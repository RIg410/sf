use crate::{
    model::{Location, Hall, WorkingHours},
    storage::LocationStore,
};
use eyre::Error;
use mongodb::bson::oid::ObjectId;
use std::{ops::Deref, sync::Arc};
use store::{Db, session::Session};
use tx_macro::tx;

#[derive(Clone)]
pub struct Locations {
    store: Arc<LocationStore>,
}

impl Locations {
    pub fn new(store: &Db) -> Self {
        Locations {
            store: Arc::new(LocationStore::new(store)),
        }
    }

    #[tx]
    pub async fn create(
        &self,
        session: &mut Session,
        name: String,
        address: String,
        working_hours: WorkingHours,
    ) -> Result<ObjectId, Error> {
        let existing = self.get_by_name(session, &name).await?;
        if existing.is_some() {
            return Err(eyre::eyre!("Location with this name already exists"));
        }

        let location = Location {
            id: ObjectId::new(),
            name,
            address,
            working_hours,
            halls: Vec::new(),
            version: 0,
        };

        self.store.insert(session, &location).await?;
        Ok(location.id)
    }

    #[tx]
    pub async fn add_hall(
        &self,
        session: &mut Session,
        location_id: ObjectId,
        hall_name: String,
    ) -> Result<ObjectId, Error> {
        let hall = Hall::new(hall_name);
        let hall_id = hall.id;
        
        self.store.add_hall(session, &location_id, &hall).await?;
        Ok(hall_id)
    }

    #[tx]
    pub async fn update_location_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        name: String,
    ) -> Result<(), Error> {
        self.store.update_name(session, &id, &name).await
    }

    #[tx]
    pub async fn update_location_address(
        &self,
        session: &mut Session,
        id: ObjectId,
        address: String,
    ) -> Result<(), Error> {
        self.store.update_address(session, &id, &address).await
    }

    #[tx]
    pub async fn remove_hall(
        &self,
        session: &mut Session,
        location_id: ObjectId,
        hall_id: ObjectId,
    ) -> Result<(), Error> {
        self.store.remove_hall(session, &location_id, &hall_id).await
    }

    #[tx]
    pub async fn update_hall_name(
        &self,
        session: &mut Session,
        location_id: ObjectId,
        hall_id: ObjectId,
        name: String,
    ) -> Result<(), Error> {
        self.store.update_hall_name(session, &location_id, &hall_id, &name).await
    }
}

impl Deref for Locations {
    type Target = LocationStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}