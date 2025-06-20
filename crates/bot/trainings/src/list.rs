use async_trait::async_trait;
use bot_core::{
    callback_data::{Calldata as _, TrainingIdCallback},
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::{day::fmt_weekday, training::fmt_training_status};
use chrono::{Datelike as _, Local};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;
use trainings::model::Filter;

use crate::view::TrainingView;

const TRAININGS_PER_PAGE: u32 = 7;

pub struct TrainingList {
    filter: Filter,
    offset: u32,
}

impl TrainingList {
    pub fn users(id: ObjectId) -> Self {
        Self {
            filter: Filter::Client(id),
            offset: 0,
        }
    }

    pub fn couches(id: ObjectId) -> Self {
        Self {
            filter: Filter::Instructor(id),
            offset: 0,
        }
    }

    pub fn programs(id: ObjectId) -> Self {
        Self {
            filter: Filter::Program(id),
            offset: 0,
        }
    }
}

#[async_trait]
impl View for TrainingList {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let (msg, keyboard) = render(ctx, self.filter.clone(), self.offset).await?;
        ctx.edit_origin(&msg, keyboard).await?;
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

async fn render(
    ctx: &mut Context,
    filter: Filter,
    offset: u32,
) -> Result<(String, InlineKeyboardMarkup)> {
    let mut msg = "🫶🏻 Тренировки:\n".to_owned();
    let mut keymap = InlineKeyboardMarkup::default();
    let trainings = ctx
        .services
        .calendar
        .find_trainings(
            &mut ctx.session,
            filter,
            TRAININGS_PER_PAGE as usize,
            offset as usize,
        )
        .await?;

    msg.push_str(
        "
➖➖➖➖➖➖➖➖➖➖➖➖➖➖➖➖
🟢\\- запись открыта 
⛔\\- тренировка отменена
🟠\\- запись закрыта 
✔️\\- тренировка прошла
🔵\\- тренировка идет
➖➖➖➖➖➖➖➖➖➖➖➖➖➖➖➖
",
    );

    let now = Local::now();
    for training in trainings.iter() {
        let mut row = vec![];
        let slot = training.get_slot();
        let start_at = slot.start_at();
        row.push(
            Callback::SelectTraining(training.id().into()).button(format!(
                "{} {} {} {}",
                fmt_training_status(
                    training.status(now),
                    training.is_processed,
                    training.is_full(),
                    training.clients.contains(&ctx.me.id)
                ),
                fmt_weekday(start_at.weekday()),
                start_at.format("%d.%m %H:%M"),
                training.name.as_str(),
            )),
        );
        keymap = keymap.append_row(row);
    }
    let mut row = vec![];

    if offset > 0 {
        row.push(Callback::Offset(offset - TRAININGS_PER_PAGE).button("⬅️"));
    }
    if (trainings.len() as u32) >= TRAININGS_PER_PAGE {
        row.push(Callback::Offset(offset + TRAININGS_PER_PAGE).button("➡️"));
    };
    keymap = keymap.append_row(row);

    Ok((msg, keymap))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    SelectTraining(TrainingIdCallback),
    Offset(u32),
}
