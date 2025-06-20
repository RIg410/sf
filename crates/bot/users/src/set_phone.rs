use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use teloxide::types::{InlineKeyboardMarkup, Message};

pub struct SetPhone {
    id: ObjectId,
}

impl SetPhone {
    pub fn new(id: ObjectId) -> SetPhone {
        SetPhone { id }
    }
}

#[async_trait]
impl View for SetPhone {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("Введите телефон", InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        let text = message.text().unwrap_or_default();
        if text.is_empty() {
            ctx.send_notification("Введите телефон").await;
            return Ok(Jmp::Stay);
        }

        ctx.services
            .users
            .set_phone(&mut ctx.session, self.id, text)
            .await?;
        ctx.delete_msg(message.id).await?;
        Ok(Jmp::Back(1))
    }
}
