use async_trait::async_trait;
use bot_core::{
    callback_data::{Calldata as _, TrainingIdCallback},
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_weekday;
use chrono::{Datelike as _, Local};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;
use trainings::model::{Filter, status::TrainingStatus};

use crate::view::TrainingView;

const TRAININGS_PER_PAGE: u32 = 7;

pub struct TrainingList {
    filter: Filter,
    offset: u32,
    mark_my: bool,
}

impl TrainingList {
    pub fn client(id: ObjectId) -> Self {
        Self {
            filter: Filter::Client(id),
            offset: 0,
            mark_my: false,
        }
    }

    pub fn couches(id: ObjectId) -> Self {
        Self {
            filter: Filter::Instructor(id),
            offset: 0,
            mark_my: true,
        }
    }

    pub fn programs(id: ObjectId) -> Self {
        Self {
            filter: Filter::Program(id),
            offset: 0,
            mark_my: true,
        }
    }
}

#[async_trait]
impl View for TrainingList {
    fn safe_point(&self) -> bool {
        true
    }
    

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut keymap = InlineKeyboardMarkup::default();
        let trainings = ctx
            .services
            .calendar
            .find_trainings(
                &mut ctx.session,
                self.filter.clone(),
                TRAININGS_PER_PAGE as usize,
                self.offset as usize,
            )
            .await?;

        let now = Local::now();
        for training in trainings.iter() {
            let slot = training.get_slot();
            let start_at = slot.start_at();

            let is_my = training.clients.contains(&ctx.me.id) && self.mark_my;
            let status = format_status(training.status(now), training.is_full(), is_my);
            let message = format!(
                "{} {} {} {}",
                status,
                fmt_weekday(start_at.weekday()),
                start_at.format("%d.%m %H:%M"),
                training.name.as_str(),
            );

            keymap =
                keymap.append_row(Callback::SelectTraining(training.id().into()).btn_row(message));
        }
        let mut row = vec![];

        if self.offset > 0 {
            row.push(Callback::Offset(self.offset - TRAININGS_PER_PAGE).button("⬅️"));
        }
        if (trainings.len() as u32) >= TRAININGS_PER_PAGE {
            row.push(Callback::Offset(self.offset + TRAININGS_PER_PAGE).button("➡️"));
        };
        keymap = keymap.append_row(row);
        ctx.edit_origin("🫶🏻 Тренировки:\n", keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectTraining(date) => Ok(TrainingView::new(date.into()).into()),
            Callback::Offset(offset) => {
                self.offset = offset;
                Ok(Jmp::Stay)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    SelectTraining(TrainingIdCallback),
    Offset(u32),
}

fn format_status(status: TrainingStatus, is_full: bool, is_my: bool) -> &'static str {
    if is_my {
        return "❤️";
    }
    match status {
        TrainingStatus::OpenToSignup { close_sign_out } => {
            if is_full {
                "🔒"
            } else if close_sign_out {
                "🔐"
            } else {
                "🟢"
            }
        }
        TrainingStatus::ClosedToSignup => "🔒",
        TrainingStatus::InProgress => "✔️",
        TrainingStatus::Cancelled => "❌",
        TrainingStatus::Finished => "✔️",
    }
}
