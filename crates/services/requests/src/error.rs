use bson::oid::ObjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Request not found")]
    RequestNotFound { id: ObjectId },
}
