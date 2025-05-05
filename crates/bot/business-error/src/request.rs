use crate::{BusinessError, Format};
use requests::error::RequestError;

impl BusinessError for RequestError {
    fn message(&self, _: Format) -> String {
        match self {
            RequestError::Eyre(_) | RequestError::MongoError(_) => "Системная ошибка".to_string(),
            RequestError::RequestNotFound { .. } => "Заявка не найдена".to_string(),
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            RequestError::Eyre(_)
            | RequestError::MongoError(_)
            | RequestError::RequestNotFound { .. } => true,
        }
    }
}
