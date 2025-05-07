use crate::schedule::group::set_date_time::TimeParts;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_dt;
use chrono::{Local, Timelike as _, Utc};
use eyre::Result;
use ident::{rooms::Room, slot::Slot, training::TrainingId};
use rights::Rule;
use teloxide::types::{InlineKeyboardMarkup, Message};
use tracing::warn;

pub struct ChangeRoom {
    id: TrainingId,
    all: bool,
}

impl ChangeRoom {
    pub fn new(id: TrainingId, all: bool) -> ChangeRoom {
        ChangeRoom { id, all }
    }
}

#[async_trait]
impl View for ChangeRoom {
    fn name(&self) -> &'static str {
        "ChangeTime"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::ChangeTrainingSlot)?;

        let msg = "Выберете помещение";
        let keymap = InlineKeyboardMarkup::default();
        let keymap = keymap.append_row(vec![
            Room::Adult.button("👨‍🏫 Зал для взрослых"),
            Room::Child.button("👶 Зал для детей"),
        ]);

        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::ChangeTrainingSlot)?;
        let room: Room = calldata!(data);

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        Ok(Jmp::Goto(
            ConfirmChangeSlot {
                id: training.id(),
                new_slot: training.get_slot().with_room(room.id()),
                all: self.all,
            }
            .widget(),
        ))
    }
}

pub struct ChangeTime {
    id: TrainingId,
    all: bool,
}

impl ChangeTime {
    pub fn new(id: TrainingId, all: bool) -> ChangeTime {
        ChangeTime { id, all }
    }
}

#[async_trait]
impl View for ChangeTime {
    fn name(&self) -> &'static str {
        "ChangeTime"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::ChangeTrainingSlot)?;

        let msg = "Введите время начала тренировки в формате HH:MM";
        let keymap = InlineKeyboardMarkup::default();
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        let msg = if let Some(msg) = msg.text() {
            msg
        } else {
            return Ok(Jmp::Stay);
        };

        let parts = match TimeParts::try_from(msg) {
            Ok(parts) => parts,
            Err(err) => {
                warn!("Invalid time format: {}", err);
                ctx.send_msg("Неверный формат времени\\.").await?;
                return Ok(Jmp::Stay);
            }
        };
        let hours = parts.0;
        let minute = parts.1;

        let start_at = if let Some(start_at) = self
            .id
            .start_at
            .with_timezone(&Local)
            .with_hour(hours)
            .and_then(|t| t.with_minute(minute))
        {
            start_at.with_timezone(&Utc)
        } else {
            ctx.send_msg("Неверный формат времени\\.").await?;
            return Ok(Jmp::Stay);
        };

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        Ok(ConfirmChangeSlot {
            id: self.id,
            new_slot: Slot::new(start_at, training.duration_min, self.id.room),
            all: self.all,
        }
        .into())
    }
}

struct ConfirmChangeSlot {
    id: TrainingId,
    new_slot: Slot,
    all: bool,
}

#[async_trait]
impl ConfirmView for ConfirmChangeSlot {
    async fn message(&self, _: &mut Context) -> Result<String> {
        Ok(if self.all {
            format!(
                "Изменить слот тренировок с *{}* *{}* на *{}* *{}*",
                fmt_dt(&self.id.start_at()),
                room_name(Room::from(self.id.room)),
                fmt_dt(&self.new_slot.start_at()),
                room_name(Room::from(self.new_slot.room())),
            )
        } else {
            format!(
                "Изменить слот тренировки с *{}* *{}* на *{}* *{}*",
                fmt_dt(&self.id.start_at()),
                room_name(Room::from(self.id.room)),
                fmt_dt(&self.new_slot.start_at()),
                room_name(Room::from(self.new_slot.room())),
            )
        })
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::ChangeTrainingSlot)?;

        ctx.services
            .calendar
            .change_slot(&mut ctx.session, self.id, self.new_slot, self.all)
            .await?;
        Ok(DoneView::ok(format!(
            "Слот тренировки изменен c *{}* *{}* на *{}* *{}*",
            fmt_dt(&self.id.start_at()),
            room_name(Room::from(self.id.room)),
            fmt_dt(&self.new_slot.start_at()),
            room_name(Room::from(self.new_slot.room()))
        ))
        .into())
    }
}

fn room_name(room: Room) -> &'static str {
    match room {
        Room::Adult => "во взрослом зале",
        Room::Child => "в детском зале",
    }
}
