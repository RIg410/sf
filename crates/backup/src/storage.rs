use std::{collections::HashMap, sync::Arc};

use bson::{doc, Bson};
use futures_util::{StreamExt as _, TryStreamExt as _};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use eyre::Result;
use store::session::{Db, Session};

pub struct BackupStorage {
    db: Arc<Db>,
}

impl BackupStorage {
    pub fn new(db: Arc<Db>) -> Self {
        BackupStorage { db }
    }

    pub async fn backup(&self, session: &mut Session) -> Result<HashMap<String, CollectionBackup>> {
        let mut collections = self.db.list_collections().await?;

        let mut backup = HashMap::new();
        while let Some(collection) = collections.next().await {
            let collection = collection?;
            let name = collection.name.clone();
            let collections: Collection<Bson> = self.db.collection(&name);
            let mut items = collections.find(doc! {}).session(&mut *session).await?;
            let items: Vec<Bson> = items.stream(&mut *session).try_collect().await?;
            backup.insert(name, CollectionBackup { data: items });
        }
        Ok(backup)
    }

    pub async fn restore(
        &self,
        backup: HashMap<String, CollectionBackup>,
        session: &mut Session,
    ) -> Result<()> {
        for (name, items) in backup {
            let collections: Collection<Bson> = self.db.collection(&name);
            collections
                .delete_many(doc! {})
                .session(&mut *session)
                .await?;
            let items = items.data;
            for item in items {
                collections.insert_one(item).session(&mut *session).await?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct CollectionBackup {
    data: Vec<Bson>,
}
