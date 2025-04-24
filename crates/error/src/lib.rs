use bson::oid::ObjectId;
use chrono::Local;
use model::{ids::DayId, training::Training, user::rate::Rate};
use thiserror::Error;
use trainings::error::TrainingError;

#[derive(Error, Debug)]
pub enum SfError {
    // common
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    // users
    #[error("User not found: {0}")]
    UserNotFound(ObjectId),
    #[error("Member not found")]
    MemberNotFound {
        user_id: ObjectId,
        member_id: ObjectId,
    },
    #[error("Wrong family member")]
    WrongFamilyMember {
        user_id: ObjectId,
        member_id: ObjectId,
    },
    #[error("User already in family")]
    UserAlreadyInFamily {
        user_id: ObjectId,
        member_id: ObjectId,
    },
    #[error("User already employee")]
    UserAlreadyEmployee { user_id: ObjectId },
    #[error("User not employee")]
    UserNotEmployee { user_id: ObjectId },
    #[error("Employee has reward")]
    EmployeeHasReward { user_id: ObjectId },
    #[error("Employee has trainings")]
    CouchHasTrainings(ObjectId),
    #[error("Employee has trainings")]
    NoRatesFound { user_id: ObjectId },
    #[error("Rate not found")]
    RateNotFound { user_id: ObjectId, rate: Rate },
    #[error("Rate already exists")]
    RateTypeAlreadyExists { user_id: ObjectId, rate: Rate },
    #[error("Request not found")]
    RequestNotFound { id: ObjectId },
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
    #[error("Training error:{0}")]
    TrainingsError(#[from] TrainingError),
}
