use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct SetAiPrompt {
    id: ObjectId,
}

impl SetAiPrompt {
    pub fn new(id: ObjectId) -> SetAiPrompt {
        SetAiPrompt { id }
    }
}

#[async_trait]
impl View for SetAiPrompt {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let ext = ctx
            .services
            .users
            .get_extension(&mut ctx.session, self.id)
            .await?;
        let msg = if let Some(prompt) = ext.ai_message_prompt {
            format!("Текущий запрос: {}\\. Введите промпт", escape(&prompt))
        } else {
            "Запрос не установлен\\. Введите промпт".to_string()
        };

        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;

        let prompt = message.text().unwrap_or_default().to_string();
        let prompt = if prompt == "-" { None } else { Some(prompt) };

        ctx.services
            .users
            .set_ai_prompt(&mut ctx.session, self.id, prompt)
            .await?;

        Ok(Jmp::Stay)
    }
}
