use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct EditProgram {
    id: ObjectId,
    edit_type: EditType,
    state: State,
}

impl EditProgram {
    pub fn new(id: ObjectId, edit_type: EditType) -> Self {
        Self {
            edit_type,
            state: State::Init,
            id,
        }
    }

    pub async fn edit_capacity(&self, ctx: &mut Context, value: u32) -> Result<Jmp> {
        ctx.ensure(Rule::EditTraining)?;
        ctx.services
            .training_adjuster
            .edit_program_capacity(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_duration(&self, ctx: &mut Context, value: u32) -> Result<Jmp> {
        ctx.ensure(Rule::EditTraining)?;
        ctx.services
            .training_adjuster
            .edit_program_duration(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_name(&self, ctx: &mut Context, value: String) -> Result<Jmp> {
        ctx.ensure(Rule::EditTraining)?;
        ctx.services
            .training_adjuster
            .edit_program_name(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_description(&self, ctx: &mut Context, value: String) -> Result<Jmp> {
        ctx.ensure(Rule::EditTraining)?;
        ctx.services
            .training_adjuster
            .edit_program_description(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }
}

#[async_trait]
impl View for EditProgram {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let keymap = InlineKeyboardMarkup::default();
        match self.edit_type {
            EditType::Capacity => {
                ctx.send_msg_with_markup("Введите новую вместимость", keymap)
                    .await?;
            }
            EditType::Duration => {
                ctx.send_msg_with_markup("Введите новую длительность", keymap)
                    .await?;
            }
            EditType::Name => {
                ctx.send_msg_with_markup("Введите новое название", keymap)
                    .await?;
            }
            EditType::Description => {
                ctx.send_msg_with_markup("Введите новое описание", keymap)
                    .await?;
            }
        }
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        match self.state {
            State::Init => {
                let text = message.text().unwrap_or_default().to_string();
                let new_value = match self.edit_type {
                    EditType::Capacity => {
                        if let Err(err) = text.parse::<NonZero<u32>>() {
                            ctx.send_msg(&format!("Неверный формат: {err}")).await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("вместимость на {text}")
                    }
                    EditType::Duration => {
                        if let Err(err) = text.parse::<NonZero<u32>>() {
                            ctx.send_msg(&format!("Неверный формат: {err}")).await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("длительность на {text}")
                    }
                    EditType::Name => format!("название на {text}"),
                    EditType::Description => format!("описание на {text}"),
                };
                self.state = State::Confirm(text);
                let mut keymap = InlineKeyboardMarkup::default();
                keymap = keymap.append_row(vec![
                    Callback::Yes.button("✅ Да"),
                    Callback::No.button("❌ Нет"),
                ]);

                ctx.send_msg_with_markup(
                    &escape(&format!("Вы уверены, что хотите изменить {new_value}?")),
                    keymap,
                )
                .await?;
            }
            State::Confirm(_) => {
                ctx.delete_msg(message.id).await?;
            }
        }

        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Yes => {
                let value = if let State::Confirm(value) = self.state.clone() {
                    value
                } else {
                    return Ok(Jmp::Stay);
                };
                match self.edit_type {
                    EditType::Capacity => self.edit_capacity(ctx, value.parse()?).await?,
                    EditType::Duration => self.edit_duration(ctx, value.parse()?).await?,
                    EditType::Name => self.edit_name(ctx, value).await?,
                    EditType::Description => self.edit_description(ctx, value).await?,
                };
                ctx.send_msg("Изменения сохранены ✅").await?;
                ctx.reset_origin();
                Ok(Jmp::Back(1))
            }
            Callback::No => {
                ctx.reset_origin();
                Ok(Jmp::Back(1))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum State {
    Init,
    Confirm(String),
}

#[derive(Clone, Copy)]
pub enum EditType {
    Capacity,
    Duration,
    Name,
    Description,
}

#[derive(Serialize, Deserialize)]
pub enum Callback {
    Yes,
    No,
}
