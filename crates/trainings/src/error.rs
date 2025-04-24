use bson::oid::ObjectId;
use thiserror::Error;

use crate::model::{id::TrainingId, status::TrainingStatus};

#[derive(Error, Debug)]
pub enum TrainingError {
    #[error("Wrong numbers of users")]
    WrongTrainingClients { training_id: TrainingId },
    // common
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    // delete training
    #[error("Training has clients")]
    TrainingHasClients(TrainingId),
    #[error("Training is processed")]
    TrainingIsProcessed(TrainingId),
    //signin
    #[error("Training not open to sign up")]
    TrainingNotOpenToSignUp(TrainingId, TrainingStatus),
    #[error("Client already signed up:{0:?} {1:?}")]
    ClientAlreadySignedUp(ObjectId, TrainingId),
    #[error("Training is full:{0:?}")]
    TrainingIsFull(TrainingId),
    #[error("Not enough balance:{0:?}")]
    NotEnoughBalance(ObjectId),
    //signout
    #[error("Training not found:{0:?}")]
    TrainingNotFound(TrainingId),
    #[error("Training is not open to sign out:{0:?}")]
    TrainingNotOpenToSignOut(TrainingId),
    #[error("Client not signed up:{0:?} {1:?}")]
    ClientNotSignedUp(ObjectId, TrainingId),
    #[error("Not enough reserved balance:{0:?}")]
    NotEnoughReservedBalance(ObjectId),
}
