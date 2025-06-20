use crate::schedule::render_time_slot_collision;

use super::{ScheduleTrainingPreset, render_msg};
use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::{DateTime, Datelike as _, Local, TimeZone, Timelike, Utc};
use eyre::{Error, Result};
use ident::slot::Slot;
use teloxide::types::{InlineKeyboardMarkup, Message};
use tracing::warn;

#[derive(Default)]
pub struct SetDateTime {
    preset: ScheduleTrainingPreset,
}

impl SetDateTime {
    pub fn new(preset: ScheduleTrainingPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for SetDateTime {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .programs
            .get_by_id(&mut ctx.session, self.preset.program_id()?)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        let request = if self.preset.day.is_none() {
            "На какой день назначить тренировку? дд\\.мм"
        } else {
            "На какое время назначить тренировку? чч\\:мм"
        };

        let msg = render_msg(ctx, &training, &self.preset, request).await?;
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;

        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.bot.delete_msg(message.id).await?;

        let msg = if let Some(msg) = message.text() {
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

        if self.preset.day.is_none() {
            if let Ok(day) = parts.to_date() {
                let mut preset = self.preset;
                preset.day = Some(day);
                return Ok(preset.into_next_view().into());
            } else {
                ctx.send_msg("Неверный формат даты\\. _дд\\.мм_").await?;
            }
        } else {
            let mut preset = self.preset;
            let day = preset.day.unwrap();
            let date_time = day.with_hour(parts.0).and_then(|d| d.with_minute(parts.1));

            if let Some(date_time) = date_time {
                let program = ctx
                    .services
                    .programs
                    .get_by_id(&mut ctx.session, self.preset.program_id()?)
                    .await?
                    .ok_or_else(|| eyre::eyre!("Training not found"))?;
                let slot = Slot::new(
                    date_time.with_timezone(&Utc),
                    program.duration_min,
                    preset.room.unwrap(),
                );

                if let Some(collision) = ctx
                    .services
                    .calendar
                    .check_time_slot(&mut ctx.session, slot, preset.is_one_time.unwrap_or(true))
                    .await?
                {
                    ctx.send_notification(&render_time_slot_collision(&collision))
                        .await;
                    preset.date_time = None;
                } else {
                    preset.date_time = Some(date_time);
                }
                return Ok(preset.into_next_view().into());
            } else {
                ctx.send_notification("Неверный формат времени\\. _чч\\:мм_")
                    .await;
            }
        }
        Ok(Jmp::Stay)
    }
}

pub struct TimeParts(pub u32, pub u32);

impl TryFrom<&str> for TimeParts {
    type Error = eyre::Error;

    fn try_from(value: &str) -> Result<Self> {
        let parts = if value.contains(":") {
            value.split(':').collect::<Vec<_>>()
        } else {
            value.split('.').collect::<Vec<_>>()
        };
        if parts.len() != 2 {
            return Err(eyre::eyre!("Invalid time format"));
        }
        let hour = parts[0].parse::<u32>()?;
        let minute = parts[1].parse::<u32>()?;
        Ok(Self(hour, minute))
    }
}

impl TimeParts {
    pub fn to_date(&self) -> Result<DateTime<Local>, Error> {
        let year = chrono::Local::now().naive_local().year_ce().1;
        Local
            .with_ymd_and_hms(
                year as i32,
                self.0.saturating_sub(1),
                self.1.saturating_sub(1),
                0,
                0,
                0,
            )
            .single()
            .ok_or_else(|| eyre::eyre!("Invalid time"))
    }
}
