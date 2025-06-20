use super::{RentPreset, render_msg};
use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use decimal::Decimal;
use eyre::Result;
use teloxide::types::{InlineKeyboardMarkup, Message};

#[derive(Default)]
pub struct SetPrice {
    preset: RentPreset,
}

impl SetPrice {
    pub fn new(preset: RentPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for SetPrice {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = render_msg(ctx, &self.preset, "Введите стоимость аренды").await?;
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;

        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        let msg = if let Some(msg) = message.text() {
            msg
        } else {
            return Ok(Jmp::Stay);
        };

        let price = match msg.parse::<Decimal>() {
            Ok(price) => price,
            Err(_) => {
                ctx.send_notification("Неверный формат стоимости\\.").await;
                return Ok(Jmp::Stay);
            }
        };
        self.preset.price = Some(price);
        Ok(self.preset.clone().into_next_view().into())
    }
}
