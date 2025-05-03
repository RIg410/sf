use super::{RentPreset, render_msg};
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use bot_viewer::rooms::fmt_room;
use eyre::Result;
use ident::rooms::Room;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

#[derive(Default)]
pub struct SetRoom {
    preset: RentPreset,
}

impl SetRoom {
    pub fn new(preset: RentPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for SetRoom {
    fn name(&self) -> &'static str {
        "SetRoom"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = render_msg(ctx, &self.preset, "В каком зале будет тренировка?").await?;
        let mut keymap = InlineKeyboardMarkup::default();
        keymap.inline_keyboard.push(vec![
            Callback::SelectRoom(Room::Adult).button(fmt_room(Room::Adult)),
            Callback::SelectRoom(Room::Child).button(fmt_room(Room::Child)),
        ]);
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectRoom(room) => {
                self.preset.room = Some(room.id());
            }
        };
        Ok(self.preset.clone().into_next_view().into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Callback {
    SelectRoom(Room),
}
