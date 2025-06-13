use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

use crate::view::LocationDetailView;

pub struct EditLocationView {
    location_id: ObjectId,
}

impl EditLocationView {
    pub fn new(location_id: ObjectId) -> Self {
        EditLocationView { location_id }
    }
}

#[async_trait]
impl View for EditLocationView {
    fn name(&self) -> &'static str {
        "EditLocationView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let location = match ctx
            .services
            .locations
            .get_by_id(&mut ctx.session, self.location_id)
            .await?
        {
            Some(location) => location,
            None => {
                ctx.send_text("❌ Локация не найдена").await?;
                return Ok(());
            }
        };

        let msg = format!(
            "✏️ *Редактирование локации*\n\n📍 Название: *{}*\n📮 Адрес: _{}_\n\nВыберите что изменить:",
            escape(&location.name),
            escape(&location.address)
        );

        let mut keymap = InlineKeyboardMarkup::default();
        
        keymap = keymap.append_row(vec![
            InlineKeyboardButton::callback(
                "📝 Изменить название",
                Callback::EditName.to_data(),
            )
        ]);
        
        keymap = keymap.append_row(vec![
            InlineKeyboardButton::callback(
                "📮 Изменить адрес",
                Callback::EditAddress.to_data(),
            )
        ]);

        keymap = keymap.append_row(vec![
            InlineKeyboardButton::callback(
                "⬅️ Назад",
                Callback::Back.to_data(),
            )
        ]);

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::System)?;

        match calldata!(data) {
            Callback::EditName => {
                return Ok(EditLocationName::new(self.location_id).into());
            }
            Callback::EditAddress => {
                return Ok(EditLocationAddress::new(self.location_id).into());
            }
            Callback::Back => {
                return Ok(LocationDetailView::new(self.location_id).into());
            }
        }

        Ok(Jmp::Stay)
    }
}

struct EditLocationName {
    location_id: ObjectId,
}

impl EditLocationName {
    fn new(location_id: ObjectId) -> Self {
        EditLocationName { location_id }
    }
}

#[async_trait]
impl View for EditLocationName {
    fn name(&self) -> &'static str {
        "EditLocationName"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("📝 Введите новое название локации:", Default::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;
        
        let new_name = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_text("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if new_name.trim().is_empty() {
            ctx.send_text("Название не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        Ok(Jmp::Next(
            ConfirmEditName {
                location_id: self.location_id,
                new_name,
            }
            .into(),
        ))
    }
}

struct ConfirmEditName {
    location_id: ObjectId,
    new_name: String,
}

#[async_trait]
impl ConfirmView for ConfirmEditName {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "📝 *Изменение названия локации*\n\nНовое название: *{}*\n\nПодтвердить изменение?",
            escape(&self.new_name)
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::System)?;
        
        match ctx
            .services
            .locations
            .update_location_name(&mut ctx.session, self.location_id, self.new_name.clone())
            .await
        {
            Ok(_) => {
                Ok(DoneView::ok(format!(
                    "✅ Название локации изменено на *{}*!",
                    escape(&self.new_name)
                ))
                .back_to(LocationDetailView::new(self.location_id))
                .into())
            }
            Err(e) => {
                Ok(DoneView::error(format!(
                    "❌ Ошибка изменения названия: {}",
                    e
                ))
                .back_to(EditLocationView::new(self.location_id))
                .into())
            }
        }
    }
}

struct EditLocationAddress {
    location_id: ObjectId,
}

impl EditLocationAddress {
    fn new(location_id: ObjectId) -> Self {
        EditLocationAddress { location_id }
    }
}

#[async_trait]
impl View for EditLocationAddress {
    fn name(&self) -> &'static str {
        "EditLocationAddress"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("📮 Введите новый адрес локации:", Default::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;
        
        let new_address = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_text("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if new_address.trim().is_empty() {
            ctx.send_text("Адрес не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        Ok(Jmp::Next(
            ConfirmEditAddress {
                location_id: self.location_id,
                new_address,
            }
            .into(),
        ))
    }
}

struct ConfirmEditAddress {
    location_id: ObjectId,
    new_address: String,
}

#[async_trait]
impl ConfirmView for ConfirmEditAddress {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "📮 *Изменение адреса локации*\n\nНовый адрес: *{}*\n\nПодтвердить изменение?",
            escape(&self.new_address)
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::System)?;
        
        match ctx
            .services
            .locations
            .update_location_address(&mut ctx.session, self.location_id, self.new_address.clone())
            .await
        {
            Ok(_) => {
                Ok(DoneView::ok(format!(
                    "✅ Адрес локации изменен на *{}*!",
                    escape(&self.new_address)
                ))
                .back_to(LocationDetailView::new(self.location_id))
                .into())
            }
            Err(e) => {
                Ok(DoneView::error(format!(
                    "❌ Ошибка изменения адреса: {}",
                    e
                ))
                .back_to(EditLocationView::new(self.location_id))
                .into())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    EditName,
    EditAddress,
    Back,
}