use super::{ScheduleTrainingPreset, render_msg};
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use program::model::Program;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use users::model::User;

#[derive(Default)]
pub struct SetInstructor {
    preset: ScheduleTrainingPreset,
}

impl SetInstructor {
    pub fn new(preset: ScheduleTrainingPreset) -> Self {
        Self { preset }
    }
}

#[async_trait]
impl View for SetInstructor {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .programs
            .get_by_id(&mut ctx.session, self.preset.program_id()?)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        let (msg, keymap) = render(ctx, &training, &self.preset).await?;
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectInstructor(instructor_id) => {
                let instructor = ctx
                    .services
                    .users
                    .get(&mut ctx.session, ObjectId::from_bytes(instructor_id))
                    .await?
                    .ok_or_else(|| eyre::eyre!("Instructor not found"))?;
                self.preset.instructor = Some(instructor.id);
                return Ok(self.preset.into_next_view().into());
            }
        }
    }
}

async fn render(
    ctx: &mut Context,
    training: &Program,
    preset: &ScheduleTrainingPreset,
) -> Result<(String, InlineKeyboardMarkup)> {
    let mut keymap = InlineKeyboardMarkup::default();

    let instructors = ctx.services.users.instructors(&mut ctx.session).await?;
    for instructor in instructors {
        keymap
            .inline_keyboard
            .push(vec![make_instructor_button(&instructor)]);
    }
    let msg = render_msg(ctx, training, preset, "🫰Выберите инструктора?").await?;
    Ok((msg, keymap))
}

fn make_instructor_button(instructor: &User) -> InlineKeyboardButton {
    let name = format!(
        "{} {}",
        instructor.name.first_name,
        instructor
            .name
            .last_name
            .as_ref()
            .unwrap_or(&"".to_string())
    );
    Callback::SelectInstructor(instructor.id.bytes()).button(name)
}

#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    SelectInstructor([u8; 12]),
}
