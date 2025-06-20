use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use program::model::Program;
use rights::Rule;
use teloxide::types::{InlineKeyboardMarkup, Message};

pub struct CreateProgram {
    state: Option<State>,
}

impl Default for CreateProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateProgram {
    pub fn new() -> Self {
        Self { state: None }
    }
}

#[async_trait]
impl View for CreateProgram {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::CreateTraining)?;
        if self.state.is_none() {
            ctx.edit_origin(
                "📝 Введите название программы:\n_оно должно быть уникально_",
                InlineKeyboardMarkup::default(),
            )
            .await?;
            self.state = Some(State::SetName(Program::default()));
        }
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.ensure(Rule::CreateTraining)?;
        let msg = if let Some(msg) = message.text() {
            msg
        } else {
            return Ok(Jmp::Stay);
        };

        let state = self
            .state
            .take()
            .ok_or_else(|| eyre::eyre!("State is missing"))?;
        self.state = Some(match state {
            State::SetName(mut training) => {
                if ctx
                    .services
                    .programs
                    .get_by_name(&mut ctx.session, msg)
                    .await?
                    .is_some()
                {
                    ctx.send_msg("Программа с таким названием уже существует")
                        .await?;
                    State::SetName(training)
                } else {
                    training.name = msg.to_string();
                    ctx.send_msg("📝 Введите описание программы").await?;
                    State::SetDescription(training)
                }
            }
            State::SetDescription(mut training) => {
                training.description = msg.to_string();
                ctx.send_msg("📝 Введите продолжительность программы в минутах")
                    .await?;
                State::SetDuration(training)
            }
            State::SetDuration(mut training) => {
                if let Ok(duration) = msg.parse::<u32>() {
                    training.duration_min = duration;
                    ctx.send_msg("📝 Введите количество мест на программе")
                        .await?;
                    State::SetCapacity(training)
                } else {
                    ctx.send_msg("Продолжительность должна быть числом").await?;
                    State::SetDuration(training)
                }
            }
            State::SetCapacity(mut program) => {
                if let Ok(capacity) = msg.parse::<u32>() {
                    program.capacity = capacity;
                    ctx.ensure(Rule::CreateTraining)?;
                    ctx.services
                        .programs
                        .create(
                            &mut ctx.session,
                            program.name,
                            program.description,
                            program.duration_min,
                            program.capacity,
                            program.tp,
                        )
                        .await?;
                    ctx.send_msg("✅ Программа создана").await?;
                    return Ok(Jmp::Back(1));
                } else {
                    ctx.send_msg("Количество мест должно быть числом").await?;
                    State::SetCapacity(program)
                }
            }
        });
        Ok(Jmp::Stay)
    }
}

#[derive(Clone, Debug)]
pub enum State {
    SetName(Program),
    SetDescription(Program),
    SetDuration(Program),
    SetCapacity(Program),
}
