use calendar::error::CalendarError;
use requests::error::RequestError;
use thiserror::Error;
use trainings::error::TrainingError;
use users::error::UserError;

#[derive(Error, Debug)]
pub enum SfError {
    // common
    #[error("Common error: {0}")]
    Eyre(#[from] eyre::Error),
    #[error("Mongo error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Calendar error:{0}")]
    CalendarError(#[from] CalendarError),
    #[error("Training error:{0}")]
    TrainingsError(#[from] TrainingError),
    #[error("User error:{0}")]
    UserError(#[from] UserError),
    #[error("Request error:{0}")]
    RequestError(#[from] RequestError),
}
