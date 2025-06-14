use crate::model::*;
use crate::storage::ImagesStore;
use base64::Engine;
use bson::oid::ObjectId;
use chrono::Utc;
use eyre::{eyre, Result};
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::ops::Deref;
use std::sync::Arc;
use store::session::Session;
use store::Db;
use tracing::{debug, info};
use tx_macro::tx;

#[derive(Clone)]
pub struct Images {
    store: Arc<ImagesStore>,
}

impl Images {
    pub fn new(db: &Arc<Db>) -> Self {
        Self {
            store: Arc::new(ImagesStore::new(db)),
        }
    }

    #[tx]
    pub async fn save_image(&self, input: ImageInput, session: &mut Session) -> Result<ObjectId> {
        let hash = Self::calculate_hash(&input.data);

        if let Some(existing) = self.store.get_by_hash(&hash, session).await? {
            debug!(
                "Image with hash {} already exists, returning existing id",
                hash
            );
            self.store
                .increment_reference(&existing.id, session)
                .await?;
            return Ok(existing.id);
        }

        let (original_img, width, height) = Self::decode_image(&input.data)?;
        let (thumbnail_data, thumb_width, thumb_height) =
            Self::create_thumbnail(&original_img, &input.mime_type)?;

        let size = input.data.len() as u64;
        let thumbnail_size = thumbnail_data.len() as u64;

        let image = Image {
            id: ObjectId::new(),
            hash,
            original_data: input.data,
            thumbnail_data,
            mime_type: input.mime_type,
            size,
            thumbnail_size,
            width,
            height,
            thumbnail_width: thumb_width,
            thumbnail_height: thumb_height,
            reference_count: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
        };

        let id = self.store.insert(image, session).await?;
        info!("Saved new image with id: {}", id);
        Ok(id)
    }

    #[tx]
    pub async fn create_reference(&self, id: &ObjectId, session: &mut Session) -> Result<()> {
        let new_count = self.store.increment_reference(id, session).await?;
        debug!(
            "Incremented reference count for image {} to {}",
            id, new_count
        );
        Ok(())
    }

    #[tx]
    pub async fn delete_reference(&self, id: &ObjectId, session: &mut Session) -> Result<()> {
        let new_count = self.store.decrement_reference(id, session).await?;
        debug!(
            "Decremented reference count for image {} to {}",
            id, new_count
        );

        if new_count == 0 {
            info!("Reference count reached 0, deleting image {}", id);
            self.store.delete(id, session).await?;
        }

        Ok(())
    }

    #[tx]
    pub async fn get_image(
        &self,
        id: &ObjectId,
        session: &mut Session,
    ) -> Result<Option<ImageData>> {
        if let Some(image) = self.store.get_by_id(id, session).await? {
            Ok(Some(ImageData {
                data: image.original_data,
                mime_type: image.mime_type,
            }))
        } else {
            Ok(None)
        }
    }

    #[tx]
    pub async fn get_thumbnail(
        &self,
        id: &ObjectId,
        session: &mut Session,
    ) -> Result<Option<ImageData>> {
        if let Some(image) = self.store.get_by_id(id, session).await? {
            Ok(Some(ImageData {
                data: image.thumbnail_data,
                mime_type: image.mime_type,
            }))
        } else {
            Ok(None)
        }
    }

    #[tx]
    pub async fn get_metadata(
        &self,
        id: &ObjectId,
        session: &mut Session,
    ) -> Result<Option<ImageMetadata>> {
        if let Some(image) = self.store.get_by_id(id, session).await? {
            Ok(Some(ImageMetadata {
                id: image.id,
                hash: image.hash,
                mime_type: image.mime_type,
                size: image.size,
                width: image.width,
                height: image.height,
                reference_count: image.reference_count,
                created_at: image.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    #[tx]
    pub async fn list_metadata(&self, session: &mut Session) -> Result<Vec<ImageMetadata>> {
        self.store.get_metadata_list(session).await
    }

    #[tx]
    pub async fn cleanup_orphaned_images(&self, session: &mut Session) -> Result<u64> {
        self.store.cleanup_orphaned(session).await
    }

    fn calculate_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        base64::engine::general_purpose::STANDARD.encode(result)
    }

    fn decode_image(data: &[u8]) -> Result<(DynamicImage, u32, u32)> {
        let img =
            image::load_from_memory(data).map_err(|e| eyre!("Failed to decode image: {}", e))?;
        let (width, height) = (img.width(), img.height());
        Ok((img, width, height))
    }

    fn create_thumbnail(img: &DynamicImage, mime_type: &str) -> Result<(Vec<u8>, u32, u32)> {
        let (orig_width, orig_height) = (img.width(), img.height());

        let (new_width, new_height) =
            if orig_width > THUMBNAIL_MAX_WIDTH || orig_height > THUMBNAIL_MAX_HEIGHT {
                let ratio_w = THUMBNAIL_MAX_WIDTH as f32 / orig_width as f32;
                let ratio_h = THUMBNAIL_MAX_HEIGHT as f32 / orig_height as f32;
                let ratio = ratio_w.min(ratio_h);

                (
                    (orig_width as f32 * ratio) as u32,
                    (orig_height as f32 * ratio) as u32,
                )
            } else {
                (orig_width, orig_height)
            };

        let thumbnail = img.resize(new_width, new_height, FilterType::Lanczos3);

        let format = match mime_type {
            "image/png" => ImageFormat::Png,
            "image/jpeg" | "image/jpg" => ImageFormat::Jpeg,
            "image/webp" => ImageFormat::WebP,
            _ => ImageFormat::Jpeg,
        };

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        thumbnail
            .write_to(&mut cursor, format)
            .map_err(|e| eyre!("Failed to encode thumbnail: {}", e))?;

        Ok((buffer, new_width, new_height))
    }
}

impl Deref for Images {
    type Target = ImagesStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
