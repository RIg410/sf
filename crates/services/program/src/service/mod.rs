use crate::{
    model::{Program, TrainingType},
    storage::ProgramStore,
};
use eyre::Error;
use mongodb::bson::oid::ObjectId;
use std::{ops::Deref, sync::Arc};
use store::{Db, session::Session};
use tx_macro::tx;

#[derive(Clone)]
pub struct Programs {
    store: Arc<ProgramStore>,
}

impl Programs {
    pub fn new(store: &Db) -> Self {
        Programs {
            store: Arc::new(ProgramStore::new(store)),
        }
    }

    #[tx]
    pub async fn create(
        &self,
        session: &mut Session,
        name: String,
        description: String,
        duration_min: u32,
        capacity: u32,
        tp: TrainingType,
    ) -> Result<(), Error> {
        let proto = Program {
            id: ObjectId::new(),
            name,
            description,
            duration_min,
            capacity,
            version: 0,
            tp,
            visible: true,
        };
        let training = self.get_by_name(session, &proto.name).await?;
        if training.is_some() {
            return Err(eyre::eyre!("Training with this name already exists"));
        }

        self.store.insert(session, &proto).await?;
        Ok(())
    }
}

impl Deref for Programs {
    type Target = ProgramStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
