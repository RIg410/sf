use super::View;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, ViewResult},
};
use eyre::{Context as _, Result, eyre};
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use teloxide::types::{InlineKeyboardMarkup, Message};

pub struct FreezeProfile {
    id: ObjectId,
    state: State,
    days: u32,
}

impl FreezeProfile {
    pub fn new(id: ObjectId) -> FreezeProfile {
        FreezeProfile {
            id,
            state: State::SetDays,
            days: 0,
        }
    }
}

#[async_trait]
impl View for FreezeProfile {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        match self.state {
            State::SetDays => {
                let user = ctx
                    .services
                    .users
                    .get(&mut ctx.session, self.id)
                    .await?
                    .ok_or_else(|| eyre!("User not found!"))?;
                let client = user.as_client()?;
                ctx.send_msg_with_markup(
                    &format!(
                        "Осталось дней заморозок:_{}_\nНа сколько дней заморозить абонемент?",
                        client.freeze_days
                    ),
                    InlineKeyboardMarkup::default(),
                )
                .await?;
            }
            State::Confirm => {
                let keymap = vec![vec![
                    Callback::Yes.button("✅ Да. Замораживаем"),
                    Callback::No.button("❌ Отмена"),
                ]];
                ctx.send_msg_with_markup(
                    &format!(
                        "Замораживаем Ваш абонемент\\. Количество дней:_{}_\nВсе верно?",
                        self.days
                    ),
                    InlineKeyboardMarkup::new(keymap),
                )
                .await?;
            }
        }
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        match self.state {
            State::SetDays => {
                let days = message.text().unwrap_or_default();
                match days.parse::<NonZero<u32>>() {
                    Ok(day) => {
                        self.state = State::Confirm;
                        self.days = day.get();
                    }
                    Err(_) => {
                        ctx.send_msg("Введите число\\.").await?;
                    }
                }
            }
            State::Confirm => {
                ctx.delete_msg(message.id).await?;
            }
        }
        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb = calldata!(data);

        match cb {
            Callback::Yes => {
                let can_freeze = ctx.has_right(Rule::FreezeUsers);

                let user = ctx
                    .services
                    .users
                    .get(&mut ctx.session, self.id)
                    .await?
                    .ok_or_else(|| eyre!("User not found!"))?;
                let client = user.as_client()?;
                if !can_freeze && client.freeze_days < self.days {
                    self.state = State::SetDays;
                    ctx.send_msg("у вас недостаточно дней заморозки").await?;
                    return Ok(Jmp::Stay);
                }

                if client.freeze.is_some() {
                    ctx.send_msg("абонемент уже заморожен").await?;
                    return Ok(Jmp::Back(1));
                }
                if !can_freeze && ctx.me.id != self.id {
                    ctx.send_msg("Нет прав").await?;
                    return Ok(Jmp::Back(1));
                }

                ctx.services
                    .users
                    .freeze(&mut ctx.session, user.id, self.days, can_freeze)
                    .await
                    .context("freeze")?;
            }
            Callback::No => {}
        }
        return Ok(Jmp::Back(1));
    }
}

#[derive(Clone, Copy)]
enum State {
    SetDays,
    Confirm,
}

#[derive(Serialize, Deserialize)]
pub enum Callback {
    Yes,
    No,
}
