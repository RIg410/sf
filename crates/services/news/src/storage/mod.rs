use crate::model::*;
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use eyre::{eyre, Result};
use mongodb::{options::UpdateOptions, Collection, IndexModel};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use store::session::Session;
use store::Db;
use tracing::info;

pub struct NewsStore {
    collection: Collection<NewsArticle>,
    latest_cache: Arc<RwLock<VecDeque<NewsArticle>>>,
    cache_size: usize,
}

impl NewsStore {
    pub fn new(db: &Arc<Db>) -> Self {
        let collection: Collection<NewsArticle> = db.collection("news");
        Self {
            collection,
            latest_cache: Arc::new(RwLock::new(VecDeque::with_capacity(LATEST_NEWS_CACHE_SIZE))),
            cache_size: LATEST_NEWS_CACHE_SIZE,
        }
    }

    pub async fn ensure_indexes(&self) -> Result<()> {
        let index = IndexModel::builder()
            .keys(doc! { "published_at": -1 })
            .build();
        self.collection.create_index(index).await?;
        Ok(())
    }

    pub async fn insert(&self, article: NewsArticle, session: &mut Session) -> Result<ObjectId> {
        let id = article.id;

        self.collection
            .insert_one(article.clone())
            .session(&mut **session)
            .await?;

        self.update_latest_cache(article).await?;

        info!("Published new article with id: {}", id);
        Ok(id)
    }

    pub async fn get_by_id(
        &self,
        id: &ObjectId,
        session: &mut Session,
    ) -> Result<Option<NewsArticle>> {
        let article = self
            .collection
            .find_one(doc! { "_id": id })
            .session(&mut **session)
            .await?;

        Ok(article)
    }

    pub async fn update_content(
        &self,
        id: &ObjectId,
        new_content: &str,
        session: &mut Session,
    ) -> Result<()> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "content_md": new_content,
                "updated_at": Utc::now()
            },
            "$inc": { "version": 1 }
        };

        let options = UpdateOptions::builder().build();
        let result = self
            .collection
            .update_one(filter, update)
            .with_options(options)
            .session(&mut **session)
            .await?;

        if result.matched_count == 0 {
            return Err(eyre!("Article not found: {}", id));
        }

        self.invalidate_cache().await?;
        Ok(())
    }

    pub async fn update_image(
        &self,
        id: &ObjectId,
        new_image_id: Option<ObjectId>,
        session: &mut Session,
    ) -> Result<Option<ObjectId>> {
        let article = self
            .get_by_id(id, session)
            .await?
            .ok_or_else(|| eyre!("Article not found: {}", id))?;

        let old_image_id = article.image_id;

        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": {
                "image_id": new_image_id,
                "updated_at": Utc::now()
            },
            "$inc": { "version": 1 }
        };

        let options = UpdateOptions::builder().build();
        let result = self
            .collection
            .update_one(filter, update)
            .with_options(options)
            .session(&mut **session)
            .await?;

        if result.matched_count == 0 {
            return Err(eyre!("Article not found: {}", id));
        }

        self.invalidate_cache().await?;
        Ok(old_image_id)
    }

    pub async fn delete(&self, id: &ObjectId, session: &mut Session) -> Result<Option<ObjectId>> {
        let article = self.get_by_id(id, session).await?;

        if let Some(art) = article {
            self.collection
                .delete_one(doc! { "_id": id })
                .session(&mut **session)
                .await?;

            self.remove_from_latest_cache(id).await?;

            info!("Deleted article: {}", id);
            return Ok(art.image_id);
        }

        Ok(None)
    }

    pub async fn get_latest(&self, limit: i64, session: &mut Session) -> Result<Vec<NewsArticle>> {
        // Check if we can serve from cache
        if limit <= self.cache_size as i64 {
            let latest = self.latest_cache.read().unwrap();
            if !latest.is_empty() && latest.len() >= limit as usize {
                return Ok(latest.iter().take(limit as usize).cloned().collect());
            }
        }

        // Fetch from database
        let mut cursor = self
            .collection
            .find(doc! {})
            .sort(doc! { "published_at": -1 })
            .limit(limit)
            .session(&mut **session)
            .await?;

        let mut articles = Vec::new();
        while let Some(article) = cursor.next(&mut **session).await {
            let article = article?;
            articles.push(article);
        }

        // Update cache if fetching the default cache size
        if limit <= self.cache_size as i64 {
            self.refresh_latest_cache(&articles).await?;
        }

        Ok(articles)
    }

    pub async fn get_summaries(
        &self,
        limit: i64,
        session: &mut Session,
    ) -> Result<Vec<NewsSummary>> {
        let articles = self.get_latest(limit, session).await?;

        Ok(articles
            .into_iter()
            .map(|art| NewsSummary {
                id: art.id,
                title: art.title,
                image_id: art.image_id,
                author_name: art.author_name,
                published_at: art.published_at,
            })
            .collect())
    }

    pub async fn get_paginated(
        &self,
        offset: usize,
        limit: usize,
        session: &mut Session,
    ) -> Result<Vec<NewsArticle>> {
        let mut cursor = self
            .collection
            .find(doc! {})
            .sort(doc! { "published_at": -1 })
            .skip(offset as u64)
            .limit(limit as i64)
            .session(&mut **session)
            .await?;

        let mut articles = Vec::new();
        while let Some(article) = cursor.next(&mut **session).await {
            let article = article?;
            articles.push(article);
        }

        Ok(articles)
    }

    async fn update_latest_cache(&self, new_article: NewsArticle) -> Result<()> {
        let mut cache = self.latest_cache.write().unwrap();

        // Insert at the beginning (newest first)
        cache.push_front(new_article);

        // Keep only the latest N articles
        while cache.len() > self.cache_size {
            cache.pop_back();
        }

        Ok(())
    }

    async fn remove_from_latest_cache(&self, id: &ObjectId) -> Result<()> {
        let mut cache = self.latest_cache.write().unwrap();
        cache.retain(|art| art.id != *id);
        Ok(())
    }

    async fn refresh_latest_cache(&self, articles: &[NewsArticle]) -> Result<()> {
        let mut cache = self.latest_cache.write().unwrap();
        cache.clear();
        for article in articles.iter().take(self.cache_size) {
            cache.push_back(article.clone());
        }
        Ok(())
    }

    async fn invalidate_cache(&self) -> Result<()> {
        let mut cache = self.latest_cache.write().unwrap();
        cache.clear();

        Ok(())
    }
}
