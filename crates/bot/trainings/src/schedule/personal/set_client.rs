use super::PersonalTrainingPreset;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use bot_viewer::fmt_phone_escape_less;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use users::model::User;

#[derive(Default)]
pub struct SetClient {
    preset: PersonalTrainingPreset,
}

impl SetClient {
    pub fn new(preset: PersonalTrainingPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for SetClient {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let preset = self.preset.instructor.unwrap_or_default();
        let (msg, keymap) = render(ctx, preset).await?;
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectClient(client) => {
                let client = ctx
                    .services
                    .users
                    .get(&mut ctx.session, ObjectId::from_bytes(client))
                    .await?
                    .ok_or_else(|| eyre::eyre!("Instructor not found"))?;
                self.preset.client = Some(client.id);
                return Ok(self.preset.into_next_view().into());
            }
        }
    }
}

async fn render(
    ctx: &mut Context,
    instructor: ObjectId,
) -> Result<(&'static str, InlineKeyboardMarkup)> {
    let mut keymap = InlineKeyboardMarkup::default();

    let clients = ctx
        .services
        .users
        .find_users_for_personal_training(&mut ctx.session, instructor)
        .await?;

    let msg = if clients.is_empty() {
        "🤷‍♂️Нет клиентов с подходящим абонементом"
    } else {
        "🫰Выберите клиента"
    };

    for client in clients {
        keymap
            .inline_keyboard
            .push(vec![make_client_button(&client)]);
    }
    Ok((msg, keymap))
}

fn make_client_button(client: &User) -> InlineKeyboardButton {
    let name = format!(
        "{} {}",
        client.name.first_name,
        client
            .phone
            .as_deref()
            .map(fmt_phone_escape_less)
            .unwrap_or_else(|| "_".to_owned())
    );
    Callback::SelectClient(client.id.bytes()).button(name)
}

#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    SelectClient([u8; 12]),
}
