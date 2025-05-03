use crate::{
    context::Context,
    widget::{Jmp, ViewResult},
};
use business_error::BusinessError;
use eyre::{Error, Result};

pub async fn handle_result(ctx: &mut Context, result: ViewResult) -> Result<Jmp, Error> {
    match result {
        Ok(jmp) => Ok(jmp),
        Err(err) => {
            if err.is_fatal() {
                Err(Error::new(err))
            } else {
                let message = err.message(business_error::Format::Markdown);
                ctx.send_notification(&message).await;
                Ok(Jmp::Stay)
            }
        }
    }
}
