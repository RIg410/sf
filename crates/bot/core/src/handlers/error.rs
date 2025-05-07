use crate::{
    views::done::DoneView,
    widget::{Jmp, ViewResult},
};
use business_error::BusinessError;
use eyre::{Error, Result};

pub fn handle_err(result: ViewResult) -> Result<Jmp, Error> {
    match result {
        Ok(jmp) => Ok(jmp),
        Err(err) => {
            if err.is_fatal() {
                Err(Error::new(err))
            } else {
                let message = err.message(business_error::Format::Markdown);
                Ok(DoneView::err(message).into())
            }
        }
    }
}
