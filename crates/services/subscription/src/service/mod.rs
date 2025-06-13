use crate::{model::Subscription, storage::SubscriptionsStore};
use decimal::Decimal;
use eyre::Error;
use std::{ops::Deref, sync::Arc};
use store::{Db, session::Session};
use thiserror::Error;
use tx_macro::tx;

#[derive(Clone)]
pub struct Subscriptions {
    pub store: Arc<SubscriptionsStore>,
}

impl Subscriptions {
    pub fn new(store: &Db) -> Self {
        Subscriptions {
            store: Arc::new(SubscriptionsStore::new(store)),
        }
    }

    pub async fn get_all(&self, session: &mut Session) -> Result<Vec<Subscription>, Error> {
        let mut cursor = self.store.cursor(session).await?;
        let mut result = Vec::new();
        while let Some(subscription) = cursor.next(session).await {
            result.push(subscription?);
        }
        Ok(result)
    }

    #[tx]
    pub async fn create_subscription(
        &self,
        session: &mut Session,
        sub: Subscription,
    ) -> Result<(), CreateSubscriptionError> {
        if self.get_by_name(session, &sub.name).await?.is_some() {
            return Err(CreateSubscriptionError::NameAlreadyExists);
        }
        if sub.items == 0 {
            return Err(CreateSubscriptionError::InvalidItems);
        }

        if sub.price < Decimal::zero() {
            return Err(CreateSubscriptionError::InvalidPrice);
        }
        self.store.insert(session, sub).await?;
        Ok(())
    }
}

impl Deref for Subscriptions {
    type Target = SubscriptionsStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

#[derive(Error, Debug)]
pub enum CreateSubscriptionError {
    #[error("Subscription with this name already exists")]
    NameAlreadyExists,
    #[error("Invalid items count")]
    InvalidItems,
    #[error("Invalid price")]
    InvalidPrice,
    #[error(transparent)]
    Common(#[from] Error),
}

impl From<mongodb::error::Error> for CreateSubscriptionError {
    fn from(err: mongodb::error::Error) -> Self {
        CreateSubscriptionError::Common(err.into())
    }
}
