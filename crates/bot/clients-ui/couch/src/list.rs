use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use users::model::User;

use crate::info::CouchInfo;

pub struct CouchingList {}

impl Default for CouchingList {
    fn default() -> Self {
        Self::new()
    }
}

impl CouchingList {
    pub fn new() -> CouchingList {
        CouchingList {}
    }
}

#[async_trait]
impl View for CouchingList {
    fn name(&self) -> &'static str {
        "CouchingList"
    }

    fn safe_point(&self) -> bool {
        true
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = "Наши инструкторы ❤️";
        let mut keymap = InlineKeyboardMarkup::default();
        let instructs = ctx.services.users.instructors(&mut ctx.session).await?;

        for instruct in instructs {
            keymap = keymap.append_row(vec![render_button(&instruct)]);
        }

        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectCouch(id) => {
                let id: ObjectId = ObjectId::from_bytes(id);
                return Ok(CouchInfo::new(id).into());
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    SelectCouch([u8; 12]),
}

fn render_button(user: &User) -> InlineKeyboardButton {
    Callback::SelectCouch(user.id.bytes()).button(user.name.first_name.to_string())
}
