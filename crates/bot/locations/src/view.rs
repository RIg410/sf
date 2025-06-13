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

use crate::{LocationsView, edit::EditLocationView};

pub struct LocationDetailView {
    location_id: ObjectId,
}

impl LocationDetailView {
    pub fn new(location_id: ObjectId) -> Self {
        LocationDetailView { location_id }
    }
}

#[async_trait]
impl View for LocationDetailView {
    fn name(&self) -> &'static str {
        "LocationDetailView"
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
                ctx.send_notification("❌ Локация не найдена").await;
                return Ok(());
            }
        };

        let mut msg = format!(
            "📍 *{}*\n\n📮 Адрес: _{}_\n\n",
            escape(&location.name),
            escape(&location.address)
        );

        if location.halls.is_empty() {
            msg.push_str("🚪 Залы: _не добавлены_\n");
        } else {
            msg.push_str(&format!("🚪 Залы \\({}\\):\n", location.halls.len()));
            for hall in &location.halls {
                msg.push_str(&format!("• *{}*\n", escape(&hall.name)));
            }
        }

        let mut keymap = InlineKeyboardMarkup::default();

        // Edit location button
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            "✏️ Редактировать",
            Callback::Edit.to_data(),
        )]);

        // Add hall button
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            "➕ Добавить зал",
            Callback::AddHall.to_data(),
        )]);

        // Hall management buttons
        for hall in &location.halls {
            keymap = keymap.append_row(vec![
                InlineKeyboardButton::callback(
                    format!("✏️ Редактировать: {}", hall.name),
                    Callback::EditHall(hall.id.bytes()).to_data(),
                ),
                InlineKeyboardButton::callback(
                    format!("🗑️ Удалить: {}", hall.name),
                    Callback::DeleteHall(hall.id.bytes()).to_data(),
                ),
            ]);
        }

        // Back button
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            "⬅️ Назад к списку",
            Callback::Back.to_data(),
        )]);

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::System)?;

        match calldata!(data) {
            Callback::Edit => Ok(EditLocationView::new(self.location_id).into()),
            Callback::AddHall => Ok(AddHallView::new(self.location_id).into()),
            Callback::EditHall(hall_id) => {
                Ok(EditHallView::new(self.location_id, ObjectId::from_bytes(hall_id)).into())
            }
            Callback::DeleteHall(hall_id) => Ok(ConfirmDeleteHall {
                location_id: self.location_id,
                hall_id: ObjectId::from_bytes(hall_id),
            }
            .into()),
            Callback::Back => Ok(LocationsView::new().into()),
        }
    }
}

pub struct AddHallView {
    location_id: ObjectId,
}

impl AddHallView {
    pub fn new(location_id: ObjectId) -> Self {
        AddHallView { location_id }
    }
}

#[async_trait]
impl View for AddHallView {
    fn name(&self) -> &'static str {
        "AddHallView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("🚪 Введите название зала:", Default::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;

        let hall_name = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_msg("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if hall_name.trim().is_empty() {
            ctx.send_msg("Название зала не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        match ctx
            .services
            .locations
            .add_hall(&mut ctx.session, self.location_id, hall_name.clone())
            .await
        {
            Ok(_) => Ok(
                DoneView::ok(format!("✅ Зал *{}* успешно добавлен", escape(&hall_name))).into(),
            ),
            Err(e) => Ok(DoneView::err(format!("❌ Ошибка добавления зала: {e}")).into()),
        }
    }
}

struct ConfirmDeleteHall {
    location_id: ObjectId,
    hall_id: ObjectId,
}

#[async_trait]
impl ConfirmView for ConfirmDeleteHall {
    async fn message(&self, ctx: &mut Context) -> Result<String> {
        // Get hall name for confirmation message
        if let Some(location) = ctx
            .services
            .locations
            .get_by_id(&mut ctx.session, self.location_id)
            .await?
        {
            if let Some(hall) = location.halls.iter().find(|h| h.id == self.hall_id) {
                return Ok(format!(
                    "❗ *Удаление зала*\n\nВы уверены, что хотите удалить зал *{}*?",
                    escape(&hall.name)
                ));
            }
        }

        Ok("❗ Удалить зал?".to_string())
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::System)?;

        match ctx
            .services
            .locations
            .remove_hall(&mut ctx.session, self.location_id, self.hall_id)
            .await
        {
            Ok(_) => Ok(DoneView::ok("✅ Зал успешно удален").into()),
            Err(e) => Ok(DoneView::err(format!("❌ Ошибка удаления зала: {e}")).into()),
        }
    }
}

pub struct EditHallView {
    location_id: ObjectId,
    hall_id: ObjectId,
}

impl EditHallView {
    pub fn new(location_id: ObjectId, hall_id: ObjectId) -> Self {
        EditHallView {
            location_id,
            hall_id,
        }
    }
}

#[async_trait]
impl View for EditHallView {
    fn name(&self) -> &'static str {
        "EditHallView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        // Get current hall name for display
        let location = match ctx
            .services
            .locations
            .get_by_id(&mut ctx.session, self.location_id)
            .await?
        {
            Some(location) => location,
            None => {
                ctx.send_msg("❌ Локация не найдена").await?;
                return Ok(());
            }
        };

        let hall_name = location
            .halls
            .iter()
            .find(|h| h.id == self.hall_id)
            .map(|h| h.name.clone())
            .unwrap_or_else(|| "Неизвестный зал".to_string());

        let msg = format!(
            "✏️ Редактирование зала *{}*\n\nВведите новое название зала:",
            escape(&hall_name)
        );
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        ctx.ensure(Rule::System)?;

        let new_name = if let Some(text) = message.text() {
            text.to_string()
        } else {
            ctx.send_msg("Введите текст").await?;
            return Ok(Jmp::Stay);
        };

        if new_name.trim().is_empty() {
            ctx.send_msg("Название зала не может быть пустым").await?;
            return Ok(Jmp::Stay);
        }

        Ok(Jmp::Next(
            ConfirmEditHall {
                location_id: self.location_id,
                hall_id: self.hall_id,
                new_name,
            }
            .into(),
        ))
    }
}

struct ConfirmEditHall {
    location_id: ObjectId,
    hall_id: ObjectId,
    new_name: String,
}

#[async_trait]
impl ConfirmView for ConfirmEditHall {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "✏️ *Переименование зала*\n\nНовое название: *{}*\n\nПодтвердить изменение?",
            escape(&self.new_name)
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::System)?;

        match ctx
            .services
            .locations
            .update_hall_name(
                &mut ctx.session,
                self.location_id,
                self.hall_id,
                self.new_name.clone(),
            )
            .await
        {
            Ok(_) => Ok(DoneView::ok(format!(
                "✅ Название зала изменено на *{}*",
                escape(&self.new_name)
            ))
            .into()),
            Err(e) => Ok(DoneView::err(format!("❌ Ошибка переименования зала: {e}")).into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    Edit,
    AddHall,
    EditHall([u8; 12]),
    DeleteHall([u8; 12]),
    Back,
}
