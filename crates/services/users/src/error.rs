use crate::model::rate::Rate;
use bson::oid::ObjectId;
use ident::user::UserIdWithName;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),

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
        member: UserIdWithName,
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
}
