use async_trait::async_trait;
use bot_core::{
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use locations::model::WorkingHours;
use rights::Rule;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct CreateLocationView;

#[async_trait]
impl View for CreateLocationView {
    fn name(&self) -> &'static str {
        "CreateLocationView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("📍 Введите название локации:", Default::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;

        let name = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_msg("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if name.trim().is_empty() {
            ctx.send_msg("Название не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        Ok(Jmp::Next(CreateLocationAddress { name }.into()))
    }
}

struct CreateLocationAddress {
    name: String,
}

#[async_trait]
impl View for CreateLocationAddress {
    fn name(&self) -> &'static str {
        "CreateLocationAddress"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = format!(
            "📍 Создание локации\n\nНазвание: *{}*\n\nВведите адрес локации:",
            escape(&self.name)
        );
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;

        let address = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_msg("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if address.trim().is_empty() {
            ctx.send_msg("Адрес не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        Ok(Jmp::Next(
            ConfirmCreateLocation {
                name: self.name.clone(),
                address,
            }
            .into(),
        ))
    }
}

struct ConfirmCreateLocation {
    name: String,
    address: String,
}

#[async_trait]
impl ConfirmView for ConfirmCreateLocation {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "📍 *Подтверждение создания локации*\n\nНазвание: *{}*\nАдрес: *{}*\n\nСоздать локацию?",
            escape(&self.name),
            escape(&self.address)
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::System)?;

        let working_hours = WorkingHours::default();

        match ctx
            .services
            .locations
            .create(
                &mut ctx.session,
                self.name.clone(),
                self.address.clone(),
                working_hours,
            )
            .await
        {
            Ok(_) => Ok(DoneView::ok(format!(
                "✅ Локация *{}* успешно создана",
                escape(&self.name)
            ))
            .into()),
            Err(e) => Ok(DoneView::err(format!("❌ Ошибка создания локации: {e}")).into()),
        }
    }
}
