use crate::program::view::ProgramView;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use eyre::{Error, Result};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

#[derive(Default)]
pub struct ProgramList;

#[async_trait]
impl View for ProgramList {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let (msg, keymap) = render(ctx).await?;
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectTraining(id) => {
                let id = ObjectId::from_bytes(id);
                Ok(ProgramView::new(id).into())
            }
        }
    }
}

async fn render(ctx: &mut Context) -> Result<(String, InlineKeyboardMarkup), Error> {
    let msg = "Тренировочные программы: 🤸🏼".to_string();
    let mut keymap = InlineKeyboardMarkup::default();

    let trainings = ctx
        .services
        .programs
        .get_all(&mut ctx.session, false)
        .await?;

    for training in trainings {
        let trainings = ctx
            .services
            .calendar
            .find_trainings(
                &mut ctx.session,
                trainings::model::Filter::Program(training.id),
                1,
                0,
            )
            .await?;
        if trainings.is_empty() {
            continue;
        }

        keymap
            .inline_keyboard
            .push(Callback::SelectTraining(training.id.bytes()).btn_row(training.name.clone()));
    }

    Ok((msg, keymap))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    SelectTraining([u8; 12]),
}
