use bson::to_document;
use dashmap::DashMap;
use eyre::Error;
use futures_util::TryStreamExt as _;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
};
use store::session::Session;
use tracing::info;

use crate::model::{Hall, Location};

const COLLECTION: &str = "locations";

pub struct LocationStore {
    pub(crate) store: Collection<Location>,
    pub(crate) location_cache: DashMap<ObjectId, Location>,
    pub(crate) name_cache: DashMap<String, ObjectId>,
}

impl LocationStore {
    pub fn new(db: &mongodb::Database) -> Self {
        let store = db.collection(COLLECTION);
        LocationStore {
            store,
            location_cache: DashMap::new(),
            name_cache: DashMap::new(),
        }
    }

    pub async fn get_by_id(
        &self,
        session: &mut Session,
        id: ObjectId,
    ) -> Result<Option<Location>, Error> {
        if !session.in_transaction() {
            if let Some(location) = self.location_cache.get(&id) {
                return Ok(Some(location.clone()));
            }
        }

        let location = self
            .store
            .find_one(doc! { "_id": id })
            .session(&mut *session)
            .await?;

        if let Some(ref location) = location {
            self.cache_location(location);
        }

        Ok(location)
    }

    pub async fn get_all(&self, session: &mut Session) -> Result<Vec<Location>, Error> {
        let mut cursor = self.store.find(doc! {}).session(&mut *session).await?;
        let locations: Vec<Location> = cursor.stream(&mut *session).try_collect().await?;

        for location in &locations {
            self.cache_location(location);
        }

        Ok(locations)
    }

    pub async fn get_by_name(
        &self,
        session: &mut Session,
        name: &str,
    ) -> Result<Option<Location>, Error> {
        if !session.in_transaction() {
            let name_lower = name.to_lowercase();
            if let Some(id) = self.name_cache.get(&name_lower) {
                if let Some(location) = self.location_cache.get(&id) {
                    return Ok(Some(location.clone()));
                }
            }
        }

        let location = self
            .store
            .find_one(doc! { "name": { "$regex": name, "$options": "i" } })
            .session(&mut *session)
            .await?;

        if let Some(ref location) = location {
            self.cache_location(location);
        }

        Ok(location)
    }

    pub async fn find_by_address(
        &self,
        session: &mut Session,
        address: &str,
    ) -> Result<Option<Location>, Error> {
        Ok(self
            .store
            .find_one(doc! { "address": { "$regex": address, "$options": "i" } })
            .session(&mut *session)
            .await?)
    }

    pub async fn insert(&self, session: &mut Session, location: &Location) -> Result<(), Error> {
        info!("Inserting location: {:?}", location);
        self.invalidate_location_cache(&location.id);

        let result = self
            .store
            .update_one(
                doc! { "name": location.name.clone() },
                doc! { "$setOnInsert": to_document(location)? },
            )
            .session(&mut *session)
            .with_options(UpdateOptions::builder().upsert(true).build())
            .await?;

        if result.upserted_id.is_none() {
            return Err(Error::msg("Location with this name already exists"));
        }
        Ok(())
    }

    pub async fn delete(&self, session: &mut Session, id: &ObjectId) -> Result<(), Error> {
        info!("Deleting location: {}", id);
        self.invalidate_location_cache(id);

        self.store
            .delete_one(doc! { "_id": id })
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn update_name(
        &self,
        session: &mut Session,
        id: &ObjectId,
        name: &str,
    ) -> Result<(), Error> {
        info!("Updating location name for {}: {}", id, name);
        self.invalidate_location_cache(id);

        self.store
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name": name }, "$inc" : { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn update_address(
        &self,
        session: &mut Session,
        id: &ObjectId,
        address: &str,
    ) -> Result<(), Error> {
        info!("Updating location address for {}: {}", id, address);
        self.invalidate_location_cache(id);

        self.store
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "address": address }, "$inc" : { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn add_hall(
        &self,
        session: &mut Session,
        location_id: &ObjectId,
        hall: &Hall,
    ) -> Result<(), Error> {
        info!("Adding hall {:?} to location {}", hall, location_id);
        self.invalidate_location_cache(location_id);

        self.store
            .update_one(
                doc! { "_id": location_id },
                doc! {
                    "$push": { "halls": to_document(hall)? },
                    "$inc" : { "version": 1 }
                },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn remove_hall(
        &self,
        session: &mut Session,
        location_id: &ObjectId,
        hall_id: &ObjectId,
    ) -> Result<(), Error> {
        info!("Removing hall {} from location {}", hall_id, location_id);
        self.invalidate_location_cache(location_id);

        self.store
            .update_one(
                doc! { "_id": location_id },
                doc! {
                    "$pull": { "halls": { "id": hall_id } },
                    "$inc" : { "version": 1 }
                },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn update_hall_name(
        &self,
        session: &mut Session,
        location_id: &ObjectId,
        hall_id: &ObjectId,
        name: &str,
    ) -> Result<(), Error> {
        info!(
            "Updating hall {} name to {} in location {}",
            hall_id, name, location_id
        );
        self.invalidate_location_cache(location_id);

        self.store
            .update_one(
                doc! { "_id": location_id, "halls.id": hall_id },
                doc! {
                    "$set": { "halls.$.name": name },
                    "$inc" : { "version": 1 }
                },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    fn cache_location(&self, location: &Location) {
        self.location_cache.insert(location.id, location.clone());
        self.name_cache
            .insert(location.name.to_lowercase(), location.id);
    }

    fn invalidate_location_cache(&self, id: &ObjectId) {
        if let Some((_, location)) = self.location_cache.remove(id) {
            self.name_cache.remove(&location.name.to_lowercase());
        }
    }
}
