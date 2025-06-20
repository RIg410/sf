use async_trait::async_trait;
use bot_core::{
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::ViewResult,
};
use bot_viewer::day::{fmt_dt, fmt_time, fmt_weekday};
use chrono::Datelike as _;
use eyre::Result;
use ident::{rooms::Room, training::TrainingId};
use rights::Rule;
use teloxide::utils::markdown::escape;

use super::slot::room_name;

pub struct ConfirmDeleteTraining {
    training: TrainingId,
    all: bool,
}

impl ConfirmDeleteTraining {
    pub fn new(training: TrainingId, all: bool) -> Self {
        Self { training, all }
    }
}

#[async_trait]
impl ConfirmView for ConfirmDeleteTraining {
    async fn message(&self, ctx: &mut Context) -> Result<String> {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.training)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        let slot = training.get_slot();

        if self.all {
            Ok(format!(
                "Удалить тренировки *{}* *{}* *{}* {} ?",
                escape(&training.name),
                fmt_weekday(slot.start_at().weekday()),
                fmt_time(&slot.start_at()),
                room_name(Room::from(slot.room())),
            ))
        } else {
            Ok(format!(
                "Удалить тренировку *{}* *{}* {} ?",
                escape(&training.name),
                fmt_dt(&slot.start_at()),
                room_name(Room::from(slot.room())),
            ))
        }
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::RemoveTraining)?;

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.training)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        if !training.is_group() {
            Err(eyre::eyre!("Can't delete personal training"))?;
        }

        let slot = training.get_slot();

        ctx.services
            .calendar
            .delete_training(&mut ctx.session, training.id(), self.all)
            .await?;
        let msg = if self.all {
            format!(
                "Тренировки *{}* *{}* *{}* {} удалены",
                escape(&training.name),
                fmt_weekday(slot.start_at().weekday()),
                fmt_time(&slot.start_at()),
                room_name(Room::from(slot.room())),
            )
        } else {
            format!(
                "Тренировка *{}* *{}* {} удалена",
                escape(&training.name),
                fmt_dt(&slot.start_at()),
                room_name(Room::from(slot.room())),
            )
        };
        Ok(DoneView::ok(msg).into())
    }
}
