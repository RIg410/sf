use crate::BusinessError;
use calendar::error::CalendarError;
use services::SfServices;
use store::session::Session;

impl BusinessError for CalendarError {
    fn message(&self) -> String {
        todo!()
    }

    fn is_fatal(&self) -> bool {
        todo!()
    }
}
