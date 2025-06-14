use crate::model::*;
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use dashmap::DashMap;
use eyre::{eyre, Result};
use mongodb::{options::UpdateOptions, Collection};
use std::sync::Arc;
use store::session::Session;
use store::Db;
use tracing::{debug, info};

pub struct ImagesStore {
    collection: Collection<Image>,
    cache: Arc<DashMap<ObjectId, Image>>,
    hash_index: Arc<DashMap<String, ObjectId>>,
}

impl ImagesStore {
    pub fn new(db: &Arc<Db>) -> Self {
        let collection: Collection<Image> = db.collection("images");
        Self {
            collection,
            cache: Arc::new(DashMap::new()),
            hash_index: Arc::new(DashMap::new()),
        }
    }

    pub async fn insert(&self, image: Image, session: &mut Session) -> Result<ObjectId> {
        let id = image.id;
        let hash = image.hash.clone();

        self.collection
            .insert_one(image.clone())
            .session(&mut **session)
            .await?;

        self.cache.insert(id, image);
        self.hash_index.insert(hash, id);

        info!("Inserted new image with id: {}", id);
        Ok(id)
    }

    pub async fn get_by_id(&self, id: &ObjectId, session: &mut Session) -> Result<Option<Image>> {
        if let Some(cached) = self.cache.get(id) {
            debug!("Cache hit for image: {}", id);
            return Ok(Some(cached.clone()));
        }

        let image = self
            .collection
            .find_one(doc! { "_id": id })
            .session(&mut **session)
            .await?;

        if let Some(ref img) = image {
            self.cache.insert(*id, img.clone());
            self.hash_index.insert(img.hash.clone(), *id);
        }

        Ok(image)
    }

    pub async fn get_by_hash(&self, hash: &str, session: &mut Session) -> Result<Option<Image>> {
        if let Some(id) = self.hash_index.get(hash) {
            return self.get_by_id(&id, session).await;
        }

        let image = self
            .collection
            .find_one(doc! { "hash": hash })
            .session(&mut **session)
            .await?;

        if let Some(ref img) = image {
            self.cache.insert(img.id, img.clone());
            self.hash_index.insert(hash.to_string(), img.id);
        }

        Ok(image)
    }

    pub async fn increment_reference(&self, id: &ObjectId, session: &mut Session) -> Result<u32> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$inc": { "reference_count": 1, "version": 1 },
            "$set": { "updated_at": Utc::now() }
        };

        let options = UpdateOptions::builder().build();
        let result = self
            .collection
            .update_one(filter, update)
            .with_options(options)
            .session(&mut **session)
            .await?;

        if result.matched_count == 0 {
            return Err(eyre!("Image not found: {}", id));
        }

        self.cache.remove(id);

        let image = self
            .get_by_id(id, session)
            .await?
            .ok_or_else(|| eyre!("Failed to retrieve updated image"))?;

        Ok(image.reference_count)
    }

    pub async fn decrement_reference(&self, id: &ObjectId, session: &mut Session) -> Result<u32> {
        let filter = doc! { "_id": id, "reference_count": { "$gt": 0 } };
        let update = doc! {
            "$inc": { "reference_count": -1, "version": 1 },
            "$set": { "updated_at": Utc::now() }
        };

        let options = UpdateOptions::builder().build();
        let result = self
            .collection
            .update_one(filter, update)
            .with_options(options)
            .session(&mut **session)
            .await?;

        if result.matched_count == 0 {
            return Err(eyre!(
                "Image not found or reference count is already 0: {}",
                id
            ));
        }

        self.cache.remove(id);

        let image = self
            .get_by_id(id, session)
            .await?
            .ok_or_else(|| eyre!("Failed to retrieve updated image"))?;

        Ok(image.reference_count)
    }

    pub async fn delete(&self, id: &ObjectId, session: &mut Session) -> Result<()> {
        if let Some(image) = self.get_by_id(id, session).await? {
            if image.reference_count > 0 {
                return Err(eyre!("Cannot delete image with reference count > 0"));
            }

            self.collection
                .delete_one(doc! { "_id": id })
                .session(&mut **session)
                .await?;

            self.cache.remove(id);
            self.hash_index.remove(&image.hash);

            info!("Deleted image: {}", id);
        }

        Ok(())
    }

    pub async fn get_metadata_list(&self, session: &mut Session) -> Result<Vec<ImageMetadata>> {
        let filter = doc! {};
        let mut cursor = self.collection.find(filter).session(&mut **session).await?;

        let mut metadata_list = Vec::new();
        while let Some(image) = cursor.next(&mut **session).await {
            let image = image?;
            let metadata = ImageMetadata {
                id: image.id,
                hash: image.hash,
                mime_type: image.mime_type,
                size: image.size,
                width: image.width,
                height: image.height,
                reference_count: image.reference_count,
                created_at: image.created_at,
            };
            metadata_list.push(metadata);
        }

        Ok(metadata_list)
    }

    pub async fn cleanup_orphaned(&self, session: &mut Session) -> Result<u64> {
        let filter = doc! { "reference_count": 0 };
        let result = self
            .collection
            .delete_many(filter)
            .session(&mut **session)
            .await?;

        info!("Cleaned up {} orphaned images", result.deleted_count);
        Ok(result.deleted_count)
    }
}
