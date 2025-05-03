use requests::error::RequestError;
use services::SfServices;

use crate::BusinessError;

impl BusinessError for RequestError {
    fn message(&self) -> String {
        todo!()
    }

    fn is_fatal(&self) -> bool {
        todo!()
    }
}
