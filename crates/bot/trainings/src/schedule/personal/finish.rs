use super::{DURATION, PersonalTrainingPreset, render_msg};
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
    preset: PersonalTrainingPreset,
}

impl Finish {
    pub fn new(preset: PersonalTrainingPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for Finish {
    fn name(&self) -> &'static str {
        "SchFinish"
    }

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
                let preset = self.preset;
                let date_time = preset
                    .date_time
                    .ok_or_else(|| eyre::eyre!("DateTime is missing"))?;
                let instructor = preset
                    .instructor
                    .ok_or_else(|| eyre::eyre!("Instructor is missing"))?;
                let client = preset
                    .client
                    .ok_or_else(|| eyre::eyre!("Client is missing"))?;
                let room = preset.room.ok_or_else(|| eyre::eyre!("Room is missing"))?;

                ctx.services
                    .booking
                    .schedule_personal_training(
                        &mut ctx.session,
                        client,
                        instructor,
                        date_time,
                        DURATION,
                        room,
                    )
                    .await?;
                ctx.send_msg("Тренировка успешно добавлена ✅").await?;
            }
            Callback::No => {
                //no-op
            }
        }

        if ctx.is_couch() && !ctx.has_right(Rule::SelectPersonalInstructor) {
            Ok(Jmp::Back(7))
        } else {
            Ok(Jmp::Back(6))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    Yes,
    No,
}
