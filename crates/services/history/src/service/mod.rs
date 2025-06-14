pub mod treasury;
pub mod user;

use crate::{
    model::{Action, ActionType, HistoryRow},
    storage::HistoryStore,
};
use chrono::{DateTime, Local, Utc};
use decimal::Decimal;
use eyre::{Error, Result};
use ident::rooms::Room;
use mongodb::{SessionCursor, bson::oid::ObjectId};
use std::{ops::Deref, sync::Arc};
use store::{Db, session::Session};
use subscription::model::{Subscription, UserSubscription};
use trainings::model::Training;
use users::model::UserName;

#[derive(Clone)]
pub struct History {
    store: Arc<HistoryStore>,
}

impl History {
    pub async fn new(store: &Db) -> Result<Self, Error> {
        Ok(History {
            store: Arc::new(HistoryStore::new(store).await?),
        })
    }

    pub async fn expire_subscription(
        &self,
        session: &mut Session,
        id: ObjectId,
        subscription: UserSubscription,
    ) -> Result<()> {
        let entry = HistoryRow::new(id, Action::ExpireSubscription { subscription });
        self.store.store(session, entry).await
    }

    pub async fn pay_reward(
        &self,
        session: &mut Session,
        user: ObjectId,
        amount: Decimal,
    ) -> Result<()> {
        let entry =
            HistoryRow::with_sub_actors(session.actor(), vec![user], Action::PayReward { amount });
        self.store.store(session, entry).await
    }

    pub async fn logs(
        &self,
        session: &mut Session,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<HistoryRow>> {
        self.store.get_logs(session, limit, offset).await
    }

    pub async fn actor_logs(
        &self,
        session: &mut Session,
        actor: ObjectId,
        limit: Option<usize>,
        offset: usize,
        actions: Vec<ActionType>,
    ) -> Result<SessionCursor<HistoryRow>> {
        self.store
            .get_actor_logs(session, actor, limit, offset, actions)
            .await
    }

    pub async fn create_user(
        &self,
        session: &mut Session,
        name: UserName,
        phone: String,
    ) -> Result<()> {
        let entry = HistoryRow::new(session.actor(), Action::CreateUser { name, phone });
        self.store.store(session, entry).await
    }

    pub async fn freeze(&self, session: &mut Session, user: ObjectId, days: u32) -> Result<()> {
        let entry =
            HistoryRow::with_sub_actors(session.actor(), vec![user], Action::Freeze { days });
        self.store.store(session, entry).await
    }

    pub async fn unfreeze(&self, session: &mut Session, user: ObjectId) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(session.actor(), vec![user], Action::Unfreeze {});
        self.store.store(session, entry).await
    }

    pub async fn change_balance(
        &self,
        session: &mut Session,
        user: ObjectId,
        amount: i32,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![user],
            Action::ChangeBalance { amount },
        );
        self.store.store(session, entry).await
    }

    pub async fn change_subscription_days(
        &self,
        session: &mut Session,
        user: ObjectId,
        delta: i32,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![user],
            Action::ChangeSubscriptionDays { delta },
        );
        self.store.store(session, entry).await
    }

    pub async fn change_reserved_balance(
        &self,
        session: &mut Session,
        user: ObjectId,
        amount: i32,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![user],
            Action::ChangeReservedBalance { amount },
        );
        self.store.store(session, entry).await
    }

    pub async fn sell_subscription(
        &self,
        session: &mut Session,
        subscription: Subscription,
        buyer: ObjectId,
        discount: Option<Decimal>,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![buyer],
            Action::SellSub {
                subscription,
                discount,
            },
        );
        self.store.store(session, entry).await
    }

    pub async fn sign_up(
        &self,
        session: &mut Session,
        user_id: ObjectId,
        start_at: DateTime<Local>,
        name: String,
        room_id: Room,
    ) -> Result<()> {
        self.store
            .store(
                session,
                HistoryRow::with_sub_actors(
                    session.actor(),
                    vec![user_id],
                    Action::SignUp {
                        start_at,
                        name,
                        room_id,
                    },
                ),
            )
            .await
    }

    pub async fn sign_out(
        &self,
        session: &mut Session,
        user_id: ObjectId,
        start_at: DateTime<Local>,
        name: String,
        room_id: Room,
    ) -> Result<()> {
        self.store
            .store(
                session,
                HistoryRow::with_sub_actors(
                    session.actor(),
                    vec![user_id],
                    Action::SignOut {
                        start_at,
                        name,
                        room_id,
                    },
                ),
            )
            .await
    }

    pub async fn block_user(
        &self,
        session: &mut Session,
        user: ObjectId,
        is_active: bool,
    ) -> Result<()> {
        self.store
            .store(
                session,
                HistoryRow::with_sub_actors(
                    session.actor(),
                    vec![user],
                    Action::BlockUser { is_active },
                ),
            )
            .await
    }

    pub async fn process_finished(&self, session: &mut Session, training: &Training) -> Result<()> {
        let sub_actors = training.clients.to_vec();

        let slot = training.id();
        let entry = HistoryRow::with_sub_actors(
            training.instructor,
            sub_actors,
            Action::FinalizedTraining {
                name: training.name.clone(),
                start_at: slot.start_at,
                room_id: Room::from(slot.room),
            },
        );
        self.store.store(session, entry).await
    }

    pub async fn process_canceled(&self, session: &mut Session, training: &Training) -> Result<()> {
        let sub_actors = training.clients.to_vec();

        let slot = training.id();
        let entry = HistoryRow::with_sub_actors(
            training.instructor,
            sub_actors,
            Action::FinalizedCanceledTraining {
                name: training.name.clone(),
                start_at: slot.start_at,
                room_id: Room::from(slot.room),
            },
        );
        self.store.store(session, entry).await
    }

    pub async fn payment(
        &self,
        session: &mut Session,
        amount: Decimal,
        description: String,
        date_time: &DateTime<Local>,
    ) -> Result<()> {
        let entry = HistoryRow::new(
            session.actor(),
            Action::Payment {
                amount,
                description,
                date_time: date_time.with_timezone(&Utc),
            },
        );
        self.store.store(session, entry).await
    }

    pub async fn deposit(
        &self,
        session: &mut Session,
        amount: Decimal,
        description: String,
        date_time: &DateTime<Local>,
    ) -> Result<()> {
        let entry = HistoryRow::new(
            session.actor(),
            Action::Deposit {
                amount,
                description,
                date_time: date_time.with_timezone(&Utc),
            },
        );
        self.store.store(session, entry).await
    }

    pub async fn remove_family_member(
        &self,
        session: &mut Session,
        main_id: ObjectId,
        member_id: ObjectId,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![main_id, member_id],
            Action::RemoveFamilyMember {},
        );
        self.store.store(session, entry).await
    }

    pub async fn add_family_member(
        &self,
        session: &mut Session,
        main_id: ObjectId,
        member_id: ObjectId,
    ) -> Result<()> {
        let entry = HistoryRow::with_sub_actors(
            session.actor(),
            vec![main_id, member_id],
            Action::AddFamilyMember {},
        );
        self.store.store(session, entry).await
    }
}

impl Deref for History {
    type Target = HistoryStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
