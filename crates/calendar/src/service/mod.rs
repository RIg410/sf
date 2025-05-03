use std::{ops::Deref, sync::Arc};

use crate::{error::CalendarError, storage::CalendarStore};
use chrono::{DateTime, Local, Utc};
use decimal::Decimal;
use eyre::{Error, Result};
use ident::{day::DayId, slot::Slot, training::TrainingId};
use mongodb::bson::oid::ObjectId;
use program::service::Programs;
use store::{Db, session::Session};
use trainings::{error::TrainingError, model::Training};
use tx_macro::tx;
use users::{log::UserLog, service::Users};

#[derive(Clone)]
pub struct Calendar<L> {
    calendar: Arc<CalendarStore>,
    users: Users<L>,
    programs: Programs,
}

impl<L: UserLog> Calendar<L> {
    pub async fn new(calendar: &Db, users: Users<L>, programs: Programs) -> Result<Self, Error> {
        Ok(Calendar {
            calendar: Arc::new(CalendarStore::new(calendar).await?),
            users,
            programs,
        })
    }

    pub async fn get_training_by_id(
        &self,
        session: &mut Session,
        id: TrainingId,
    ) -> Result<Option<Training>, Error> {
        let day = self.get_day(session, DayId::from(id.start_at)).await?;
        Ok(day.training.iter().find(|slot| slot.id() == id).cloned())
    }

    #[tx]
    pub async fn update_training_name(
        &self,
        session: &mut Session,
        id: TrainingId,
        name: &str,
    ) -> Result<(), Error> {
        self.calendar.change_name(session, id, name).await?;
        Ok(())
    }

    #[tx]
    pub async fn change_slot(
        &self,
        session: &mut Session,
        id: TrainingId,
        new_slot: Slot,
        all: bool,
    ) -> Result<(), CalendarError> {
        if DayId::from(id) != new_slot.day_id() {
            return Err(CalendarError::DayIdMismatch {
                old: DayId::from(id),
                new: new_slot.day_id(),
            });
        }

        let mut training = self
            .get_training_by_id(session, id)
            .await?
            .ok_or(TrainingError::TrainingNotFound(id))?;

        if training.is_processed {
            return Err(TrainingError::TrainingIsProcessed(training.full_name()).into());
        }

        training.set_slot(new_slot);
        self.calendar.delete_training(session, id).await?;
        let collision = self.check_time_slot(session, new_slot, true).await?;
        if let Some(collision) = collision {
            return Err(CalendarError::TimeSlotCollision(collision));
        }
        self.calendar.add_training(session, &training).await?;

        let day_id = DayId::from(training.get_slot().start_at());
        if all {
            let mut cursor = self.calendar.week_days_after(session, day_id).await?;
            while let Some(day) = cursor.next(session).await {
                let mut day = day?;
                let training = day.training.iter_mut().find(|slot| slot.id == training.id);
                if let Some(training) = training {
                    if training.is_processed {
                        return Err(TrainingError::TrainingIsProcessed(training.full_name()).into());
                    }
                    self.calendar
                        .delete_training(session, training.id())
                        .await?;

                    let new_slot = new_slot.with_day(training.day_id());
                    training.set_slot(new_slot);

                    let collision = self.check_time_slot(session, new_slot, true).await?;
                    if let Some(collision) = dbg!(collision) {
                        return Err(CalendarError::TimeSlotCollision(collision));
                    }
                    self.calendar.add_training(session, training).await?;
                }
            }
        }

        Ok(())
    }

    #[tx]
    pub async fn change_couch(
        &self,
        session: &mut Session,
        id: TrainingId,
        new_couch: ObjectId,
        all: bool,
    ) -> Result<(), Error> {
        if let Some(training) = self.get_training_by_id(session, id).await? {
            self.calendar.change_couch(session, id, new_couch).await?;

            let day_id = DayId::from(training.get_slot().start_at());
            if all {
                let mut cursor = self.calendar.week_days_after(session, day_id).await?;
                while let Some(day) = cursor.next(session).await {
                    let day = day?;
                    let training = day.training.iter().find(|slot| slot.id == training.id);
                    if let Some(training) = training {
                        self.calendar
                            .change_couch(session, training.id(), new_couch)
                            .await?;
                    }
                }
            }
        } else {
            return Err(eyre::eyre!("Training not found:{:?}", id));
        }

        Ok(())
    }

    #[tx]
    pub async fn delete_training(
        &self,
        session: &mut Session,
        id: TrainingId,
        all: bool,
    ) -> Result<(), CalendarError> {
        if let Some(training) = self.get_training_by_id(session, id).await? {
            if !training.clients.is_empty() {
                return Err(TrainingError::TrainingHasClients(training.full_name()).into());
            }

            self.calendar.delete_training(session, id).await?;

            let day_id = DayId::from(training.get_slot().start_at());
            if all {
                let mut cursor = self.calendar.week_days_after(session, day_id).await?;
                while let Some(day) = cursor.next(session).await {
                    let day = day?;
                    let training = day.training.iter().find(|slot| slot.id == training.id);
                    if let Some(training) = training {
                        if !training.clients.is_empty() {
                            return Err(TrainingError::TrainingHasClients(training.full_name()).into());
                        }
                        self.calendar
                            .delete_training(session, training.id())
                            .await?;
                    }
                }
            }
        } else {
            return Err(TrainingError::TrainingNotFound(id).into());
        }

        Ok(())
    }

    #[tx]
    pub async fn schedule_rent(
        &self,
        session: &mut Session,
        start_at: DateTime<Local>,
        duration_min: u32,
        room: ObjectId,
        price: Decimal,
        renter: String,
    ) -> Result<(), CalendarError> {
        let slot = Slot::new(start_at.with_timezone(&Utc), duration_min, room);
        let collision = self.check_time_slot(session, slot, true).await?;
        if let Some(collision) = collision {
            return Err(CalendarError::TimeSlotCollision(collision));
        }

        let name = format!("аренда:{}-{}", renter, duration_min);
        let description = format!(
            "арендатор: {}; продолжительность: {};",
            renter, duration_min
        );
        let training = Training::new_rent(start_at, room, duration_min, name, description, price);

        self.calendar.add_training(session, &training).await?;
        Ok(())
    }

    pub async fn schedule_personal_training(
        &self,
        session: &mut Session,
        client: ObjectId,
        instructor: ObjectId,
        start_at: DateTime<Local>,
        duration_min: u32,
        room: ObjectId,
    ) -> Result<TrainingId, CalendarError> {
        let instructor = self
            .users
            .get(session, instructor)
            .await?
            .ok_or_else(|| CalendarError::InstructorNotFound(instructor))?;
        if !instructor.is_couch() {
            return Err(CalendarError::InstructorHasNoRights(instructor.id));
        }
        let client = self
            .users
            .get(session, client)
            .await?
            .ok_or(CalendarError::ClientNotFound(client))?;

        let slot = Slot::new(start_at.with_timezone(&Utc), duration_min, room);
        let collision = self.check_time_slot(session, slot, true).await?;
        if let Some(collision) = collision {
            return Err(CalendarError::TimeSlotCollision(collision));
        }

        let name = format!(
            "Инди:{}/{}",
            client.name.first_name, instructor.name.first_name
        );
        let description = instructor
            .employee
            .map(|e| e.description.clone())
            .unwrap_or_default();
        let training = Training::new_personal(
            start_at,
            room,
            instructor.id,
            duration_min,
            name,
            description,
        );

        self.calendar.add_training(session, &training).await?;
        Ok(training.id())
    }

    #[tx]
    pub async fn schedule_group(
        &self,
        session: &mut Session,
        program_id: ObjectId,
        start_at: DateTime<Local>,
        room: ObjectId,
        instructor: ObjectId,
        is_one_time: bool,
    ) -> Result<(), CalendarError> {
        let program = self
            .programs
            .get_by_id(session, program_id)
            .await?
            .ok_or_else(|| CalendarError::ProgramNotFound(program_id))?;

        let instructor = self
            .users
            .get(session, instructor)
            .await?
            .ok_or_else(|| CalendarError::InstructorNotFound(instructor))?;
        if !instructor.is_couch() {
            return Err(CalendarError::InstructorHasNoRights(instructor.id));
        }

        let day_id = DayId::from(start_at);
        let slot = Slot::new(start_at.with_timezone(&Utc), program.duration_min, room);
        let collision = self.check_time_slot(session, slot, is_one_time).await?;
        if let Some(collision) = collision {
            return Err(CalendarError::TimeSlotCollision(collision));
        }

        let mut training = Training::new_group(program, start_at, instructor.id, is_one_time, room);
        if !training.status(Local::now()).can_sign_in() {
            return Err(CalendarError::TooCloseToStart { start_at });
        }

        self.calendar.add_training(session, &training).await?;

        if !is_one_time {
            let mut cursor = self.calendar.week_days_after(session, day_id).await?;
            while let Some(day) = cursor.next(session).await {
                let day = day?;
                training = Training::with_day_and_training(day.day_id(), training);
                self.calendar.add_training(session, &training).await?;
            }
        }

        Ok(())
    }

    pub async fn check_time_slot(
        &self,
        session: &mut Session,
        slot: Slot,
        is_one_time: bool,
    ) -> Result<Option<Training>> {
        let day_id = slot.day_id();
        let day = self.get_day(session, day_id).await?;
        for training in day.training {
            if training.get_slot().has_conflict(&slot) {
                return Ok(Some(training));
            }
        }

        if !is_one_time {
            let mut cursor = self.calendar.week_days_after(session, day_id).await?;
            while let Some(day) = cursor.next(session).await {
                let day = day?;
                let slot = slot.with_day(day.day_id());
                for training in day.training {
                    if training.get_slot().has_conflict(&slot) {
                        return Ok(Some(training));
                    }
                }
            }
        }

        Ok(None)
    }
}

impl<L: UserLog> Calendar<L> {
    pub async fn edit_duration(
        &self,
        session: &mut Session,
        program_id: ObjectId,
        duration: u32,
    ) -> Result<()> {
        let mut cursor = self
            .calendar
            .find_with_program_id(session, program_id)
            .await?;
        while let Some(day) = cursor.next(session).await {
            let mut day = day?;
            for training in &mut day.training {
                if training.proto_id == program_id {
                    training.duration_min = duration;
                }
            }

            if day.has_conflict() {
                return Err(eyre::eyre!("Conflicts found"));
            }

            self.calendar
                .update_duration_in_day(session, day.id, program_id, duration)
                .await?;
        }

        Ok(())
    }
}

impl<L> Deref for Calendar<L> {
    type Target = CalendarStore;

    fn deref(&self) -> &Self::Target {
        &self.calendar
    }
}
