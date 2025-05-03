use bson::oid::ObjectId;
use ident::{training::{TrainingFullName, TrainingId}, user::UserIdWithName};
use thiserror::Error;

use crate::model::status::TrainingStatus;

#[derive(Error, Debug)]
pub enum TrainingError {
    // common
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    // delete training
    #[error("Training has clients")]
    TrainingHasClients(TrainingFullName),
    #[error("Training is processed")]
    TrainingIsProcessed(TrainingFullName),
    //signin
    #[error("Training not open to sign up")]
    TrainingNotOpenToSignUp(TrainingFullName, TrainingStatus),
    #[error("Client already signed up:{0:?} {1:?}")]
    ClientAlreadySignedUp(UserIdWithName, TrainingFullName),
    #[error("Training is full:{0:?}")]
    TrainingIsFull(TrainingFullName),
    #[error("Not enough balance:{0:?}")]
    NotEnoughBalance(UserIdWithName),
    //signout
    #[error("Training not found:{0:?}")]
    TrainingNotFound(TrainingId),
    #[error("Training is not open to sign out:{0:?}")]
    TrainingNotOpenToSignOut(TrainingFullName),
    #[error("Client not signed up:{0:?} {1:?}")]
    ClientNotSignedUp(UserIdWithName, TrainingFullName),
    #[error("Not enough reserved balance:{0:?}")]
    NotEnoughReservedBalance(UserIdWithName),
}
