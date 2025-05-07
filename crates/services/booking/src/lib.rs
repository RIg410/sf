use bson::oid::ObjectId;
use calendar::{error::CalendarError, service::Calendar};
use chrono::{DateTime, Local};
use history::service::History;
use ident::{rooms::Room, training::TrainingId};
use payer::{FindFor, SubscriptionResolver as _};
use store::session::Session;
use trainings::{
    error::TrainingError,
    model::{Training, status::TrainingStatus},
};
use tx_macro::tx;
use users::{log::UserLog, service::Users};

pub mod payer;

pub struct Booking<L> {
    calendar: Calendar<L>,
    users: Users<L>,
    history: History,
}

impl<L: UserLog> Booking<L> {
    pub fn new(calendar: Calendar<L>, users: Users<L>, history: History) -> Self {
        Booking {
            calendar,
            users,
            history,
        }
    }

    #[tx]
    pub async fn schedule_personal_training(
        &self,
        session: &mut Session,
        client: ObjectId,
        instructor: ObjectId,
        start_at: DateTime<Local>,
        duration_min: u32,
        room: ObjectId,
    ) -> Result<(), CalendarError> {
        let id = self
            .calendar
            .schedule_personal_training(session, client, instructor, start_at, duration_min, room)
            .await?;
        self.sign_up_txless(session, id, client, true).await?;
        Ok(())
    }

    #[tx]
    pub async fn sign_up(
        &self,
        session: &mut Session,
        id: TrainingId,
        client: ObjectId,
        forced: bool,
    ) -> Result<(), CalendarError> {
        let training: Training = self
            .calendar
            .get_training_by_id(session, id)
            .await?
            .ok_or_else(|| TrainingError::TrainingNotFound(id))?;
        let status = training.status(Local::now());
        if !forced && !status.can_sign_in() {
            return Err(
                TrainingError::TrainingNotOpenToSignUp(training.full_name(), status).into(),
            );
        }

        if training.is_processed {
            return Err(
                TrainingError::TrainingNotOpenToSignUp(training.full_name(), status).into(),
            );
        }
        let mut user = self
            .users
            .get(session, client)
            .await?
            .ok_or_else(|| CalendarError::ClientNotFound(client))?;

        if training.clients.contains(&client) {
            return Err(TrainingError::ClientAlreadySignedUp(
                user.id_with_name(),
                training.full_name(),
            )
            .into());
        }

        if training.clients.len() as u32 >= training.capacity {
            return Err(TrainingError::TrainingIsFull(training.full_name()).into());
        }

        let user_id = user.id;

        self.users.resolve_family(session, &mut user).await?;
        let mut payer = user.payer_mut()?;

        if training.tp.is_not_free() {
            let subscription = payer
                .find_subscription(FindFor::Lock, &training)
                .ok_or_else(|| TrainingError::NotEnoughBalance)?;

            if !subscription.lock_balance() {
                return Err(TrainingError::NotEnoughBalance.into());
            }
            self.users.update(session, &mut payer).await?;
        }

        self.calendar
            .sign_up(session, training.id(), client)
            .await?;
        self.history
            .sign_up(
                session,
                user_id,
                training.get_slot().start_at(),
                training.name,
                Room::from(training.room),
            )
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn sign_out(
        &self,
        session: &mut Session,
        id: TrainingId,
        client: ObjectId,
        forced: bool,
    ) -> Result<(), CalendarError> {
        let training = self
            .calendar
            .get_training_by_id(session, id)
            .await?
            .ok_or_else(|| TrainingError::TrainingNotFound(id))?;
        self.sign_out_tx_less(session, &training, client, forced)
            .await?;
        if training.tp.is_personal() {
            self.calendar
                .delete_training_txless(session, training.id(), false)
                .await?;
        }
        Ok(())
    }

    pub(crate) async fn sign_out_tx_less(
        &self,
        session: &mut Session,
        training: &Training,
        client: ObjectId,
        forced: bool,
    ) -> Result<(), CalendarError> {
        let status = training.status(Local::now());
        if !forced && !status.can_sign_out() {
            return Err(TrainingError::TrainingNotOpenToSignOut(training.full_name()).into());
        }
        if training.is_processed {
            return Err(TrainingError::TrainingNotOpenToSignOut(training.full_name()).into());
        }

        let mut user = self
            .users
            .get(session, client)
            .await?
            .ok_or_else(|| CalendarError::ClientNotFound(client))?;

        if !training.clients.contains(&client) {
            return Err(TrainingError::ClientNotSignedUp(
                user.id_with_name(),
                training.full_name(),
            )
            .into());
        }

        self.users.resolve_family(session, &mut user).await?;
        let user_id = user.id;
        let mut payer = user.payer_mut()?;

        if training.tp.is_not_free() {
            let sub = payer
                .find_subscription(FindFor::Unlock, training)
                .ok_or_else(|| TrainingError::NotEnoughReservedBalance)?;
            if !sub.unlock_balance() {
                return Err(TrainingError::NotEnoughReservedBalance.into());
            }
            self.users.update(session, &mut payer).await?;
        }
        self.calendar
            .sign_out(session, training.id(), client)
            .await?;
        self.history
            .sign_out(
                session,
                user_id,
                training.get_slot().start_at(),
                training.name.clone(),
                Room::from(training.room),
            )
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn cancel_training(
        &self,
        session: &mut Session,
        training: &Training,
    ) -> Result<Vec<ObjectId>, CalendarError> {
        let training = self
            .calendar
            .get_training_by_id(session, training.id())
            .await?
            .ok_or_else(|| TrainingError::TrainingNotFound(training.id()))?;

        for client in &training.clients {
            self.sign_out_tx_less(session, &training, *client, true)
                .await?;
        }

        self.calendar
            .set_cancel_flag(session, training.id(), true)
            .await?;

        if training.tp.is_personal() || training.tp.is_sub_rent() {
            self.calendar
                .delete_training_txless(session, training.id(), false)
                .await?;
        }
        Ok(training.clients)
    }

    #[tx]
    pub async fn restore_training(
        &self,
        session: &mut Session,
        training: &Training,
    ) -> Result<(), CalendarError> {
        if training.status(Local::now()) != TrainingStatus::Cancelled {
            return Err(CalendarError::TrainingNotCancelled);
        }
        let day = self.calendar.get_day(session, training.day_id()).await?;

        if let Some(collision) = day.has_conflict_with(training.get_slot()) {
            return Err(CalendarError::TimeSlotCollision(collision.full_name()));
        }

        self.calendar
            .set_cancel_flag(session, training.id(), false)
            .await?;
        Ok(())
    }
}
