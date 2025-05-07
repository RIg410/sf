use bson::oid::ObjectId;
use ident::{day::DayId, training::TrainingFullName};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalendarError {
    // common
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    //new training
    #[error("Program not found:{0}")]
    ProgramNotFound(ObjectId),
    #[error("Instructor not found:{0}")]
    InstructorNotFound(ObjectId),
    #[error("Client not found:{0}")]
    ClientNotFound(ObjectId),
    #[error("Instructor has no rights:{0}")]
    InstructorHasNoRights(ObjectId),
    #[error("Time slot collision:{0:?}")]
    TimeSlotCollision(TrainingFullName),
    #[error("Day id mismatch")]
    DayIdMismatch { old: DayId, new: DayId },
    #[error("Training error:{0:?}")]
    TrainingError(#[from] trainings::error::TrainingError),
    #[error("Training is not cancelled")]
    TrainingNotCancelled,
}
