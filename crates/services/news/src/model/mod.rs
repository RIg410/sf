use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub content_md: String,
    pub image_id: Option<ObjectId>,
    pub author_id: ObjectId,
    pub author_name: String,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsInput {
    pub title: String,
    pub content_md: String,
    pub image_id: Option<ObjectId>,
    pub author_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsSummary {
    pub id: ObjectId,
    pub title: String,
    pub image_id: Option<ObjectId>,
    pub author_name: String,
    pub published_at: DateTime<Utc>,
}

pub const LATEST_NEWS_CACHE_SIZE: usize = 20;
