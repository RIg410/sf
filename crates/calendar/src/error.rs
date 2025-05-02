use bson::oid::ObjectId;
use chrono::Local;
use ident::day::DayId;
use thiserror::Error;
use trainings::model::Training;

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
    #[error("Too close to start")]
    TooCloseToStart { start_at: chrono::DateTime<Local> },
    #[error("Time slot collision:{0:?}")]
    TimeSlotCollision(Training),
    #[error("Day id mismatch")]
    DayIdMismatch { old: DayId, new: DayId },
    #[error("Training error:{0:?}")]
    TrainingError(#[from] trainings::error::TrainingError),
}
