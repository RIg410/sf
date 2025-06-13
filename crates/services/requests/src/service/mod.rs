use std::{ops::Deref, sync::Arc};

use crate::error::RequestError;
use crate::model::RemindLater;
use crate::model::Request;
use crate::model::RequestHistoryRow;
use crate::storage::RequestStore;
use bson::oid::ObjectId;
use chrono::Utc;
use eyre::Error;
use eyre::Result;
use ident::source::Source;
use store::Db;
use store::session::Session;
use tx_macro::tx;
use users::log::UserLog;
use users::model::sanitize_phone;
use users::service::Users;

#[derive(Clone)]
pub struct Requests<L> {
    requests: Arc<RequestStore>,
    users: Users<L>,
}

impl<L: UserLog> Requests<L> {
    pub async fn new(store: &Db, users: Users<L>) -> Result<Self, Error> {
        Ok(Requests {
            requests: Arc::new(RequestStore::new(store).await?),
            users,
        })
    }

    #[tx]
    pub async fn update_come_from(
        &self,
        session: &mut Session,
        id: ObjectId,
        come_from: Source,
        comment: String,
    ) -> Result<(), RequestError> {
        if let Some(mut request) = self.requests.get(session, id).await? {
            request.history.push(RequestHistoryRow {
                comment: request.comment.clone(),
                date_time: request.modified,
            });
            request.modified = Utc::now();
            request.comment = comment;
            request.come_from = come_from;
            self.requests.update(session, &request).await?;

            let user = self
                .users
                .get_by_phone(session, &sanitize_phone(&request.phone))
                .await?;
            if let Some(user) = user {
                self.users
                    .update_come_from(session, user.id, come_from)
                    .await?;
            }
        } else {
            return Err(RequestError::RequestNotFound { id });
        }
        Ok(())
    }

    #[tx]
    pub async fn add_comment(
        &self,
        session: &mut Session,
        id: ObjectId,
        comment: String,
    ) -> Result<(), RequestError> {
        if let Some(mut request) = self.requests.get(session, id).await? {
            request.history.push(RequestHistoryRow {
                comment: request.comment.clone(),
                date_time: request.modified,
            });
            request.modified = Utc::now();
            request.comment = comment;
            self.requests.update(session, &request).await?;
        } else {
            return Err(RequestError::RequestNotFound { id });
        }
        Ok(())
    }

    #[tx]
    pub async fn add_notification(
        &self,
        session: &mut Session,
        id: ObjectId,
        remember_later: Option<RemindLater>,
    ) -> Result<(), RequestError> {
        if let Some(mut request) = self.requests.get(session, id).await? {
            request.remind_later = remember_later;
            self.requests.update(session, &request).await?;
        } else {
            return Err(RequestError::RequestNotFound { id });
        }
        Ok(())
    }

    #[tx]
    pub async fn create_request(
        &self,
        session: &mut Session,
        phone: String,
        come_from: Source,
        comment: String,
        first_name: Option<String>,
        last_name: Option<String>,
        remember_later: Option<RemindLater>,
    ) -> Result<()> {
        let phone = sanitize_phone(&phone);
        let user = self.users.get_by_phone(session, &phone).await?;
        if let Some(user) = user {
            self.users
                .update_come_from(session, user.id, come_from)
                .await?;
        }
        if let Some(mut request) = self.requests.get_by_phone(session, &phone).await? {
            request.remind_later = remember_later;
            request.history.push(RequestHistoryRow {
                comment: request.comment.clone(),
                date_time: request.modified,
            });
            request.modified = Utc::now();
            request.comment = comment;
            request.come_from = come_from;
            self.requests.update(session, &request).await?;
        } else {
            self.requests
                .create(
                    session,
                    Request::new(
                        phone,
                        comment,
                        come_from,
                        first_name,
                        last_name,
                        remember_later,
                    ),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn come_from(&self, session: &mut Session, phone: &str) -> Result<Source, Error> {
        let phone = sanitize_phone(phone);
        self.requests
            .get_by_phone(session, &phone)
            .await
            .map(|r| r.map(|r| r.come_from).unwrap_or_default())
    }
}

impl<L> Deref for Requests<L> {
    type Target = RequestStore;

    fn deref(&self) -> &Self::Target {
        &self.requests
    }
}
