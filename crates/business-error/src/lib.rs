use services::error::SfError;

mod calendar;
mod request;
mod sell_subscription;
mod trainings;
mod user;

pub trait BusinessError {
    fn message(&self, format: Format) -> String;
    fn is_fatal(&self) -> bool;
}

impl BusinessError for SfError {
    fn message(&self, format: Format) -> String {
        match self {
            SfError::Eyre(_) | SfError::MongoError(_) => "Системная ошибка".to_string(),
            SfError::CalendarError(err) => err.message(format),
            SfError::TrainingsError(err) => err.message(format),
            SfError::UserError(err) => err.message(format),
            SfError::RequestError(err) => err.message(format),
            SfError::SellSubscriptionError(err) => err.message(format),
            SfError::ParseIntError(_) | SfError::ParseDecimalError(_) => {
                "Неверный формат числа".to_string()
            }
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            SfError::Eyre(_) | SfError::MongoError(_) => true,
            SfError::CalendarError(err) => err.is_fatal(),
            SfError::TrainingsError(err) => err.is_fatal(),
            SfError::UserError(err) => err.is_fatal(),
            SfError::RequestError(err) => err.is_fatal(),
            SfError::SellSubscriptionError(err) => err.is_fatal(),
            SfError::ParseIntError(_) | SfError::ParseDecimalError(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Text,
    Markdown,
}
