use crate::{
    error::UserError,
    log::UserLog,
    model::{
        User, UserName,
        extension::{Birthday, UserExtension},
        role::RoleType,
        sanitize_phone,
    },
    storage::UserStore,
};
use ::ai::Ai;
use chrono::{DateTime, Local};
use eyre::{Context as _, Error, Result, eyre};
use ident::source::Source;
use mongodb::{SessionCursor, bson::oid::ObjectId};
use rights::Rule;
use std::{ops::Deref, sync::Arc};
use store::{Db, session::Session};
use thiserror::Error;
use tracing::info;
use tx_macro::tx;

pub mod comments;
pub mod family;
pub mod statistics;
pub mod subscription;

#[derive(Clone)]
pub struct Users<L> {
    pub(super) store: Arc<UserStore>,
    pub(super) logs: L,
    pub(crate) _ai: Ai,
}

impl<L: UserLog> Users<L> {
    pub async fn new(store: &Db, logs: L, ai: Ai) -> Result<Self, Error> {
        Ok(Users {
            store: Arc::new(UserStore::new(store).await?),
            logs,
            _ai: ai,
        })
    }

    #[tx]
    pub async fn migrate_users(&self, session: &mut Session) -> Result<()> {
        self.store.migrate_users(session).await?;
        Ok(())
    }

    pub async fn get_user(&self, session: &mut Session, id: ObjectId) -> Result<User, UserError> {
        self.get(session, id)
            .await
            .context("get_user")?
            .ok_or_else(|| UserError::UserNotFound(id))
    }

    #[tx]
    pub async fn create(
        &self,
        session: &mut Session,
        tg_id: i64,
        name: UserName,
        phone: String,
        come_from: Source,
    ) -> Result<ObjectId> {
        let phone = sanitize_phone(&phone);
        let is_first_user = self.store.count(session, false).await? == 0;
        let role = if is_first_user {
            RoleType::Admin
        } else {
            RoleType::Client
        };

        let user = self.get_by_tg_id(session, tg_id).await?;
        if user.is_some() {
            return Err(eyre::eyre!("User {} already exists", tg_id));
        }

        let user = self.get_by_phone(session, &phone).await?;
        if let Some(user) = user {
            self.store.set_tg_id(session, user.id, tg_id).await?;
            self.store.set_name(session, user.id, name).await?;
            Ok(user.id)
        } else {
            let mut user = User::new(tg_id, name.clone(), Some(phone.clone()), role);
            user.as_client_mut()?.come_from = come_from;

            let id = user.id;
            self.store.insert(session, user).await?;
            self.logs.create_user(session, name, phone).await?;
            self.store
                .update_extension(
                    session,
                    UserExtension {
                        id,
                        birthday: None,
                        notification_mask: Default::default(),
                        ai_message_prompt: None,
                        comments: Default::default(),
                    },
                )
                .await?;
            Ok(id)
        }
    }

    pub async fn create_uninit(
        &self,
        session: &mut Session,
        phone: String,
        first_name: String,
        last_name: Option<String>,
        come_from: Source,
    ) -> Result<User> {
        let phone = sanitize_phone(&phone);

        let user = self.get_by_phone(session, &phone).await?;
        if user.is_some() {
            return Err(eyre::eyre!("User {} already exists", phone));
        }

        let user_name = UserName {
            tg_user_name: None,
            first_name,
            last_name,
        };

        let mut user = User::new(-1, user_name.clone(), Some(phone.clone()), RoleType::Client);
        user.as_client_mut()?.come_from = come_from;

        self.store.insert(session, user.clone()).await?;
        self.logs.create_user(session, user_name, phone).await?;
        self.store
            .update_extension(
                session,
                UserExtension {
                    birthday: None,
                    id: user.id,
                    notification_mask: Default::default(),
                    ai_message_prompt: None,
                    comments: Default::default(),
                },
            )
            .await?;
        Ok(user)
    }

    pub async fn find(
        &self,
        session: &mut Session,
        query: &str,
        offset: u64,
        limit: u64,
        employee: Option<bool>,
        only_with_subscriptions: bool,
    ) -> Result<SessionCursor<User>> {
        let keywords = query.split_whitespace().collect::<Vec<_>>();
        self.store
            .find(
                session,
                &keywords,
                offset,
                limit,
                employee,
                only_with_subscriptions,
            )
            .await
    }

    #[tx]
    pub async fn set_user_birthday(
        &self,
        session: &mut Session,
        id: ObjectId,
        date: DateTime<Local>,
        forced: bool,
    ) -> Result<(), SetDateError> {
        let mut user = self
            .store
            .get_extension(session, id)
            .await
            .map_err(SetDateError::Common)?;
        if !forced && user.birthday.is_some() {
            return Err(SetDateError::AlreadySet);
        }
        user.birthday = Some(Birthday::new(date));
        self.store
            .update_extension(session, user)
            .await
            .map_err(SetDateError::Common)?;
        Ok(())
    }

    #[tx]
    pub async fn set_ai_prompt(
        &self,
        session: &mut Session,
        id: ObjectId,
        prompt: Option<String>,
    ) -> Result<()> {
        let mut user = self
            .store
            .get_extension(session, id)
            .await
            .map_err(SetDateError::Common)?;
        user.ai_message_prompt = prompt;
        self.store.update_extension(session, user).await?;
        Ok(())
    }

    #[tx]
    pub async fn edit_user_rule(
        &self,
        session: &mut Session,
        id: ObjectId,
        rule: Rule,
        is_active: bool,
    ) -> Result<()> {
        if is_active {
            self.store.add_rule(session, id, &rule).await?;
            info!("Adding rule {:?} to user {}", rule, id);
        } else {
            self.store.remove_rule(session, id, &rule).await?;
            info!("Removing rule {:?} from user {}", rule, id);
        }

        Ok(())
    }

    #[tx]
    pub async fn set_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        first_name: &str,
        last_name: &str,
    ) -> Result<()> {
        self.store.set_first_name(session, id, first_name).await?;
        self.store.set_last_name(session, id, last_name).await?;
        Ok(())
    }

    #[tx]
    pub async fn set_phone(&self, session: &mut Session, id: ObjectId, phone: &str) -> Result<()> {
        let phone = sanitize_phone(phone);
        self.store.set_phone(session, id, &phone).await?;
        Ok(())
    }
}

impl<L: UserLog> Users<L> {
    #[tx]
    pub async fn unfreeze(&self, session: &mut Session, id: ObjectId) -> Result<()> {
        let user = self
            .store
            .get(session, id)
            .await?
            .ok_or_else(|| eyre!("User not found"))?;
        let client = user.as_client()?;
        if client.freeze.is_none() {
            return Ok(());
        }

        self.logs.unfreeze(session, user.id).await?;
        self.store.unfreeze(session, id).await
    }

    #[tx]
    pub async fn freeze(
        &self,
        session: &mut Session,
        id: ObjectId,
        days: u32,
        force: bool,
    ) -> Result<(), UserError> {
        let user = self
            .store
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;

        let client = user.as_client()?;
        if !force && client.freeze_days < days {
            return Err(UserError::InsufficientFreezeDays);
        }
        if client.freeze.is_some() {
            return Err(UserError::UserAlreadyFrozen);
        }

        self.logs.freeze(session, user.id, days).await?;
        self.store.freeze(session, id, days, force).await?;
        Ok(())
    }

    #[tx]
    pub async fn block_user(
        &self,
        session: &mut Session,
        id: ObjectId,
        is_active: bool,
    ) -> Result<()> {
        self.logs.block_user(session, id, is_active).await?;
        self.store.block_user(session, id, is_active).await?;
        Ok(())
    }
}

impl<L> Deref for Users<L> {
    type Target = UserStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

#[derive(Debug, Error)]
pub enum SetDateError {
    #[error("User not found")]
    UserNotFound,
    #[error("Birthday already set")]
    AlreadySet,
    #[error(transparent)]
    Common(eyre::Error),
}

impl From<mongodb::error::Error> for SetDateError {
    fn from(e: mongodb::error::Error) -> Self {
        SetDateError::Common(e.into())
    }
}

impl From<eyre::Error> for SetDateError {
    fn from(e: eyre::Error) -> Self {
        SetDateError::Common(e)
    }
}
