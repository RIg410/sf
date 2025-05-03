use thiserror::Error;

#[derive(Error, Debug)]
pub enum SellSubscriptionError {
    #[error("Subscription not found")]
    SubscriptionNotFound,
    #[error("Subscription already sold")]
    SubscriptionAlreadySold,
    #[error("User not found")]
    UserNotFound,
    #[error("{0:?}")]
    Common(#[from] eyre::Error),
}

impl From<mongodb::error::Error> for SellSubscriptionError {
    fn from(value: mongodb::error::Error) -> Self {
        SellSubscriptionError::Common(value.into())
    }
}
