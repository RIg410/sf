use sales::error::SellSubscriptionError;

use crate::BusinessError;

impl BusinessError for SellSubscriptionError {
    fn message(&self, _: crate::Format) -> String {
        match self {
            SellSubscriptionError::SubscriptionNotFound => "Абонемент не найден".to_string(),
            SellSubscriptionError::SubscriptionAlreadySold => "Абонемент уже продан".to_string(),
            SellSubscriptionError::UserNotFound => "Пользователь не найден".to_string(),
            SellSubscriptionError::Common(_) => "Системная ошибка".to_string(),
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            SellSubscriptionError::SubscriptionNotFound => true,
            SellSubscriptionError::SubscriptionAlreadySold => false,
            SellSubscriptionError::UserNotFound => true,
            SellSubscriptionError::Common(_) => true,
        }
    }
}
