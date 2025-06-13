use bson::to_document;
use eyre::Error;
use futures_util::TryStreamExt as _;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
    options::UpdateOptions,
};
use store::session::Session;

use crate::model::{Hall, Location};

const COLLECTION: &str = "locations";

pub struct LocationStore {
    pub(crate) store: Collection<Location>,
}

impl LocationStore {
    pub fn new(db: &mongodb::Database) -> Self {
        let store = db.collection(COLLECTION);
        LocationStore { store }
    }

    pub async fn get_by_id(
        &self,
        session: &mut Session,
        id: ObjectId,
    ) -> Result<Option<Location>, Error> {
        Ok(self
            .store
            .find_one(doc! { "_id": id })
            .session(&mut *session)
            .await?)
    }

    pub async fn get_all(&self, session: &mut Session) -> Result<Vec<Location>, Error> {
        let mut cursor = self.store.find(doc! {}).session(&mut *session).await?;
        Ok(cursor.stream(&mut *session).try_collect().await?)
    }

    pub async fn get_by_name(
        &self,
        session: &mut Session,
        name: &str,
    ) -> Result<Option<Location>, Error> {
        Ok(self
            .store
            .find_one(doc! { "name": { "$regex": name, "$options": "i" } })
            .session(&mut *session)
            .await?)
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
}
