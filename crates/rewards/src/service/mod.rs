use crate::storage::RewardsStore;
use eyre::Error;
use std::{ops::Deref, sync::Arc};
use store::Db;

#[derive(Clone)]
pub struct Rewards {
    store: Arc<RewardsStore>,
}

impl Rewards {
    pub async fn new(db: &Db) -> Result<Self, Error> {
        Ok(Rewards {
            store: Arc::new(RewardsStore::new(db).await?),
        })
    }
}

impl Deref for Rewards {
    type Target = RewardsStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
