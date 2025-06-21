use crate::list::TrainingList;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use bot_viewer::training::fmt_training_type;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

pub struct ProgramView {
    id: ObjectId,
}

impl ProgramView {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }

    async fn find_training(&mut self) -> ViewResult {
        Ok(TrainingList::programs(self.id).into())
    }
}

#[async_trait]
impl View for ProgramView {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .programs
            .get_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        let text = format!(
            "
        🧘*Тренировка*: {}
        *Продолжительность*: {}мин
        *Вместимость*: {}
        [Описание]({})
        {}
        ",
            escape(&training.name),
            training.duration_min,
            training.capacity,
            escape(&training.description),
            fmt_training_type(training.tp),
        );
        let keymap = InlineKeyboardMarkup::default()
            .append_row(Callback::FindTraining.btn_row("Найти тренировку"));

        ctx.edit_origin(&text, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        let calldata = calldata!(data);
        match calldata {
            Callback::FindTraining => self.find_training().await,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    FindTraining,
}
