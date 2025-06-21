use super::{RentPreset, render_msg};
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

#[derive(Default)]
pub struct Finish {
    preset: RentPreset,
}

impl Finish {
    pub fn new(preset: RentPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for Finish {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = render_msg(ctx, &self.preset, "Все верно?").await?;
        let keymap = vec![vec![
            Callback::Yes.button("✅ Сохранить"),
            Callback::No.button("❌ Отмена"),
        ]];
        ctx.edit_origin(&msg, InlineKeyboardMarkup::new(keymap))
            .await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Yes => {
                ctx.ensure(Rule::SchedulePersonalTraining)?;
                let preset = self.preset.clone();
                let date_time = preset
                    .date_time
                    .ok_or_else(|| eyre::eyre!("DateTime is missing"))?;
                let room = preset.room.ok_or_else(|| eyre::eyre!("Room is missing"))?;
                let duration = preset
                    .duration
                    .ok_or_else(|| eyre::eyre!("Duration is missing"))?
                    .num_minutes() as u32;
                let price = preset
                    .price
                    .ok_or_else(|| eyre::eyre!("Price is missing"))?;
                let renter = preset
                    .renter
                    .ok_or_else(|| eyre::eyre!("Renter is missing"))?;

                ctx.services
                    .calendar
                    .schedule_rent(&mut ctx.session, date_time, duration, room, price, renter)
                    .await?;
                ctx.send_msg("Тренировка успешно добавлена ✅").await?;
            }
            Callback::No => {
                //no-op
            }
        }
        Ok(Jmp::Back(7))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    Yes,
    No,
}
