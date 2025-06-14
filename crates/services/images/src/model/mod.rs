use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub hash: String,
    pub original_data: Vec<u8>,
    pub thumbnail_data: Vec<u8>,
    pub mime_type: String,
    pub size: u64,
    pub thumbnail_size: u64,
    pub width: u32,
    pub height: u32,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub reference_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageMetadata {
    pub id: ObjectId,
    pub hash: String,
    pub mime_type: String,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub reference_count: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Clone)]
pub struct ImageInput {
    pub data: Vec<u8>,
    pub mime_type: String,
}

pub const THUMBNAIL_MAX_WIDTH: u32 = 300;
pub const THUMBNAIL_MAX_HEIGHT: u32 = 300;
