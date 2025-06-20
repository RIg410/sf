use super::View;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, ViewResult},
};
use decimal::Decimal;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct EditSubscription {
    id: ObjectId,
    edit_type: EditType,
    state: State,
}

impl EditSubscription {
    pub fn new(id: ObjectId, edit_type: EditType) -> Self {
        Self {
            edit_type,
            state: State::Init,
            id,
        }
    }

    pub async fn edit_price(&self, ctx: &mut Context, value: Decimal) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_price(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_items(&self, ctx: &mut Context, value: u32) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_items(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_freeze_days(&self, ctx: &mut Context, value: u32) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_freeze_days(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_expiration_days(&self, ctx: &mut Context, value: u32) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_expiration_days(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_can_buy_by_user(&self, ctx: &mut Context, value: bool) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_can_buy_by_user(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }

    pub async fn edit_name(&self, ctx: &mut Context, value: String) -> Result<Jmp> {
        ctx.ensure(Rule::EditSubscription)?;
        ctx.services
            .subscriptions
            .edit_name(&mut ctx.session, self.id, value)
            .await?;
        Ok(Jmp::Stay)
    }
}

#[async_trait]
impl View for EditSubscription {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::EditSubscription)?;
        if State::Init == self.state {
            let mut keymap = InlineKeyboardMarkup::default();
            match self.edit_type {
                EditType::Name => {
                    ctx.send_msg_with_markup("Введите новое название", keymap)
                        .await?;
                }
                EditType::Price => {
                    ctx.send_msg_with_markup("Введите новую цену", keymap)
                        .await?;
                }
                EditType::Items => {
                    ctx.send_msg_with_markup("Введите новое количество занятий", keymap)
                        .await?;
                }
                EditType::FreezeDays => {
                    ctx.send_msg_with_markup("Введите новое количество дней заморозки", keymap)
                        .await?;
                }
                EditType::ExpirationDays => {
                    ctx.send_msg_with_markup("Введите новое количество дней действия", keymap)
                        .await?;
                }
                EditType::CanBuyByUser => {
                    keymap = keymap.append_row(vec![
                        Callback::Yes.button("✅ Да"),
                        Callback::No.button("❌ Нет"),
                    ]);
                    ctx.send_msg_with_markup("Выберите, можно ли покупать подписку", keymap)
                        .await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        match self.state {
            State::Init => {
                let text = message.text().unwrap_or_default().to_string();
                let new_value = match self.edit_type {
                    EditType::Items => {
                        if let Err(err) = text.parse::<NonZero<u32>>() {
                            ctx.send_msg(&format!("Неверный формат: {err}")).await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("количество занятий на {text}")
                    }
                    EditType::Price => {
                        if let Err(err) = text.parse::<Decimal>() {
                            ctx.send_msg(&format!("Неверный формат: {err}")).await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("цену на {text}")
                    }
                    EditType::Name => format!("название на {text}"),
                    EditType::FreezeDays => {
                        if text.parse::<u32>().is_err() {
                            ctx.send_msg("Неверный формат").await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("количество дней заморозки на {text}")
                    }
                    EditType::ExpirationDays => {
                        if let Err(err) = text.parse::<NonZero<u32>>() {
                            ctx.send_msg(&format!("Неверный формат: {err}")).await?;
                            return Ok(Jmp::Stay);
                        }
                        format!("количество дней действия на {text}")
                    }
                    EditType::CanBuyByUser => {
                        ctx.delete_msg(message.id).await?;
                        return Ok(Jmp::Stay);
                    }
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
                    EditType::Price => self.edit_price(ctx, value.parse()?).await?,
                    EditType::Items => self.edit_items(ctx, value.parse()?).await?,
                    EditType::Name => self.edit_name(ctx, value).await?,
                    EditType::FreezeDays => self.edit_freeze_days(ctx, value.parse()?).await?,
                    EditType::ExpirationDays => {
                        self.edit_expiration_days(ctx, value.parse()?).await?
                    }
                    EditType::CanBuyByUser => self.edit_can_buy_by_user(ctx, true).await?,
                };
                ctx.send_msg("Изменения сохранены ✅").await?;
                ctx.reset_origin();
                Ok(Jmp::Back(1))
            }
            Callback::No => {
                if self.edit_type == EditType::CanBuyByUser {
                    self.edit_can_buy_by_user(ctx, false).await?;
                }

                ctx.reset_origin();
                Ok(Jmp::Back(1))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum State {
    Init,
    Confirm(String),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EditType {
    Price,
    Name,
    Items,
    FreezeDays,
    ExpirationDays,
    CanBuyByUser,
}

#[derive(Serialize, Deserialize)]
pub enum Callback {
    Yes,
    No,
}
