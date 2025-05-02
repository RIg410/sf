use eyre::Result;
use std::sync::Arc;
use store::session::Db;

const DB_NAME: &str = "ledger_db";

#[derive(Clone)]
pub struct Storage {
    db: Arc<Db>,
}

impl Storage {
    pub async fn new(uri: &str) -> Result<Self> {
        let db = Db::new(uri, DB_NAME).await?;

        Ok(Storage { db: Arc::new(db) })
    }
}

impl AsRef<Arc<Db>> for Storage {
    fn as_ref(&self) -> &Arc<Db> {
        &self.db
    }
}
