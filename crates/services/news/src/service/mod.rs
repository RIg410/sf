use crate::model::*;
use crate::storage::NewsStore;
use bson::oid::ObjectId;
use chrono::Utc;
use eyre::{eyre, Result};
use history::service::History;
use images::Images;
use std::ops::Deref;
use std::sync::Arc;
use store::session::Session;
use store::Db;
use tracing::{debug, info};
use tx_macro::tx;
use users::service::Users;

pub struct News {
    store: Arc<NewsStore>,
    images: Images,
    users: Users<History>,
}

impl News {
    pub async fn new(db: &Arc<Db>, images: Images, users: Users<History>) -> Result<Self> {
        let store = Arc::new(NewsStore::new(db));
        store.ensure_indexes().await?;

        Ok(Self {
            store,
            images,
            users,
        })
    }

    #[tx]
    pub async fn publish(&self, input: NewsInput, session: &mut Session) -> Result<ObjectId> {
        // Get author name
        let author = self
            .users
            .get(session, input.author_id)
            .await?
            .ok_or_else(|| eyre!("Author not found: {}", input.author_id))?;

        // Create reference to image if provided
        if let Some(image_id) = input.image_id {
            self.images.create_reference(&image_id, session).await?;
        }

        let article = NewsArticle {
            id: ObjectId::new(),
            title: input.title,
            content_md: input.content_md,
            image_id: input.image_id,
            author_id: input.author_id,
            author_name: author.name.to_string(),
            published_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
        };

        let id = self.store.insert(article, session).await?;
        info!("Published news article: {}", id);
        Ok(id)
    }

    #[tx]
    pub async fn delete(&self, id: &ObjectId, session: &mut Session) -> Result<()> {
        let old_image_id = self.store.delete(id, session).await?;

        // Remove image reference if there was one
        if let Some(image_id) = old_image_id {
            self.images.delete_reference(&image_id, session).await?;
        }

        info!("Deleted news article: {}", id);
        Ok(())
    }

    #[tx]
    pub async fn update_content(
        &self,
        id: &ObjectId,
        new_content: &str,
        session: &mut Session,
    ) -> Result<()> {
        self.store.update_content(id, new_content, session).await?;
        debug!("Updated content for article: {}", id);
        Ok(())
    }

    #[tx]
    pub async fn update_image(
        &self,
        id: &ObjectId,
        new_image_id: Option<ObjectId>,
        session: &mut Session,
    ) -> Result<()> {
        let old_image_id = self.store.update_image(id, new_image_id, session).await?;

        // Handle image references
        if let Some(old_id) = old_image_id {
            self.images.delete_reference(&old_id, session).await?;
        }

        if let Some(new_id) = new_image_id {
            self.images.create_reference(&new_id, session).await?;
        }

        debug!("Updated image for article: {}", id);
        Ok(())
    }

    pub async fn get_by_id(
        &self,
        id: &ObjectId,
        session: &mut Session,
    ) -> Result<Option<NewsArticle>> {
        self.store.get_by_id(id, session).await
    }

    pub async fn get_latest(&self, limit: i64, session: &mut Session) -> Result<Vec<NewsArticle>> {
        self.store.get_latest(limit, session).await
    }

    pub async fn get_latest_summaries(
        &self,
        limit: i64,
        session: &mut Session,
    ) -> Result<Vec<NewsSummary>> {
        self.store.get_summaries(limit, session).await
    }

    pub async fn get_paginated(
        &self,
        offset: usize,
        limit: usize,
        session: &mut Session,
    ) -> Result<Vec<NewsArticle>> {
        self.store.get_paginated(offset, limit, session).await
    }
}

impl Deref for News {
    type Target = NewsStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
