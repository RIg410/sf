use crate::widget::ViewResult;
use crate::Calldata;
use crate::{
    context::Context,
    widget::{Jmp, View},
};
use async_trait::async_trait;
use eyre::Result;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;


#[async_trait]
trait OnConfirm {
    async fn call(&self, ctx: &mut Context) -> ViewResult;
}

pub struct Confirm {
    msg: String,
    on_confirm: Box<dyn OnConfirm + Send + Sync + 'static>,
    cancel_back_steps: usize,
}

impl Confirm {
    pub fn new(msg: String, cancel_back_steps: usize,) -> Self {
        todo!()
    }
}

#[async_trait]
impl View for Confirm {
    fn name(&self) -> &'static str {
        "Confirm"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(vec![
            ConfirmCallback::Confirm.button("✅ Подтвердить"),
            ConfirmCallback::Cancel.button("❌ Отмена"),
        ]);
        ctx.edit_origin(&self.msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb: ConfirmCallback = if let Some(cb) = Calldata::from_data(data) {
            cb
        } else {
            return Ok(Jmp::Stay);
        };

        match cb {
            ConfirmCallback::Confirm => self.on_confirm.call(ctx).await,
            ConfirmCallback::Cancel => Ok(Jmp::Back(self.cancel_back_steps)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ConfirmCallback {
    Confirm,
    Cancel,
}
