use crate::Calldata;
use crate::widget::ViewResult;
use crate::{
    context::Context,
    widget::{Jmp, View},
};
use async_trait::async_trait;
use eyre::Result;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

#[async_trait]
pub trait ConfirmView {
    async fn message(&self, ctx: &mut Context) -> Result<String>;

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult;
}

#[async_trait]
impl<V> View for V
where
    V: ConfirmView + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "Confirm"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(vec![
            ConfirmCallback::Confirm.button("✅ Да"),
            ConfirmCallback::Cancel.button("❌ Нет"),
        ]);
        let msg = self.message(ctx).await?;
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb: ConfirmCallback = if let Some(cb) = Calldata::from_data(data) {
            cb
        } else {
            return Ok(Jmp::Stay);
        };

        match cb {
            ConfirmCallback::Confirm => self.on_confirm(ctx).await,
            ConfirmCallback::Cancel => Ok(Jmp::ToSafePoint),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ConfirmCallback {
    Confirm,
    Cancel,
}
