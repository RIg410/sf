use crate::BusinessError;
use services::SfServices;
use store::session::Session;
use users::error::UserError;

impl BusinessError for UserError {
    fn message(&self) -> String {
        todo!()
    }

    fn is_fatal(&self) -> bool {
        todo!()
    }
}
