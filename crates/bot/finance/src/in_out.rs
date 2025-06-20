use std::{mem, str::FromStr as _};

use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use decimal::Decimal;
use eyre::Result;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct TreasuryOp {
    state: State,
    io: Op,
}
impl TreasuryOp {
    pub fn new(io: Op) -> Self {
        Self {
            state: State::Description,
            io,
        }
    }
}

#[async_trait]
impl View for TreasuryOp {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut text = format!("{}\n{}", self.io.render(), self.state.render());
        let mut keymap = InlineKeyboardMarkup::default();

        match self.state {
            State::Description => {
                text.push_str("\nВведите описание платежа:");
            }
            State::Amount(_) => {
                text.push_str("\nВведите сумму платежа:");
            }
            State::DateTime(_, _) => {
                text.push_str("\nВведите дату платежа: \\d\\.m\\.Y H:M");
            }
            State::Finish(_, _, _) => {
                text.push_str("\nВсе верно?");
                keymap = keymap.append_row(vec![
                    InlineKeyboardButton::callback("✅ Сохранить", Callback::Save.to_data()),
                    InlineKeyboardButton::callback("❌ Отмена", Callback::Back.to_data()),
                ]);
            }
        }

        ctx.send_msg_with_markup(&text, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        let text = if let Some(msg) = message.text() {
            msg
        } else {
            return Ok(Jmp::Stay);
        };

        let state = mem::take(&mut self.state);
        self.state = match state {
            State::Description => State::Amount(text.to_string()),
            State::Amount(des) => {
                if let Ok(amount) = u64::from_str(text) {
                    let amount = Decimal::int(amount as i64);
                    if ctx.has_right(Rule::FinanceHistoricalDate) {
                        State::DateTime(des, amount)
                    } else {
                        State::Finish(des, amount, Local::now())
                    }
                } else {
                    ctx.send_msg("Введите корректное число").await?;
                    State::Amount(des)
                }
            }
            State::DateTime(des, amount) => {
                if text == "-" {
                    State::Finish(des, amount, Local::now())
                } else {
                    let dt = NaiveDateTime::parse_from_str(text, "%d.%m.%Y %H:%M")
                        .ok()
                        .and_then(|dt| Local.from_local_datetime(&dt).single());
                    if let Some(dt) = dt {
                        State::Finish(des, amount, dt)
                    } else {
                        ctx.send_msg("Введите корректную дату").await?;
                        State::DateTime(des, amount)
                    }
                }
            }
            State::Finish(d, a, dt) => {
                ctx.delete_msg(message.id).await?;
                State::Finish(d, a, dt)
            }
        };
        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Save => match &self.state {
                State::Finish(description, amount, date) => match self.io {
                    Op::Deposit => {
                        ctx.services
                            .treasury
                            .deposit(&mut ctx.session, *amount, description.to_string(), date)
                            .await?;
                        ctx.send_msg("✅ Платеж сохранен").await?;
                        Ok(Jmp::Back(1))
                    }
                    Op::Payment => {
                        ctx.services
                            .treasury
                            .payment(&mut ctx.session, *amount, description.to_string(), date)
                            .await?;
                        ctx.send_msg("✅ Платеж сохранен").await?;
                        Ok(Jmp::Back(1))
                    }
                },
                _ => {
                    ctx.send_msg("Заполните все поля").await?;
                    self.state = State::Description;
                    Ok(Jmp::Stay)
                }
            },
            Callback::Back => Ok(Jmp::Back(1)),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    Save,
    Back,
}

#[derive(Default, Clone)]
enum State {
    #[default]
    Description,
    Amount(String),
    DateTime(String, Decimal),
    Finish(String, Decimal, DateTime<Local>),
}

impl State {
    pub fn render(&self) -> String {
        match self {
            State::Description => format!(
                "📝Описание:_❓_\n💲Сумма:❓\nДата:_{}_",
                Local::now().format("%d/%m/%Y %H:%M")
            ),
            State::Amount(description) => format!(
                "📝Описание:_{}_\n💲Сумма:❓\nДата:_{}_",
                escape(description),
                Local::now().format("%d/%m/%Y %H:%M")
            ),
            State::DateTime(description, amount) => format!(
                "📝Описание:_{}_\n💲Сумма:_{}_\nДата:_{}_",
                escape(description),
                amount.to_string().replace(".", ","),
                Local::now().format("%d/%m/%Y %H:%M")
            ),
            State::Finish(description, amount, date) => {
                format!(
                    "📝Описание:_{}_\n💲Сумма:_{}_\nДата:_{}_",
                    escape(description),
                    amount.to_string().replace(".", ","),
                    date.format("%d/%m/%Y %H:%M")
                )
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Op {
    Deposit,
    Payment,
}

impl Op {
    pub fn render(&self) -> &str {
        match self {
            Op::Deposit => "🤑Внести средства",
            Op::Payment => "💳Оплатить",
        }
    }
}
