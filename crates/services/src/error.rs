use calendar::error::CalendarError;
use decimal::ParseDecimalError;
use requests::error::RequestError;
use sales::error::SellSubscriptionError;
use thiserror::Error;
use trainings::error::TrainingError;
use users::error::UserError;

#[derive(Error, Debug)]
pub enum SfError {
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
    #[error("Sell subscription error:{0}")]
    SellSubscriptionError(#[from] SellSubscriptionError),
    #[error("ParseInt error:{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseDecimal error:{0}")]
    ParseDecimalError(#[from] ParseDecimalError),
}
