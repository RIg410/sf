use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::{Local, TimeZone as _, Utc};
use decimal::Decimal;
use eyre::{Error, Result};
use mongodb::bson::oid::ObjectId;
use teloxide::types::{InlineKeyboardMarkup, Message};
use users::model::rate::{Interval, Rate};

use super::new::ConfirmCreationRate;

pub struct FixRateAmount {
    old_rate: Option<Rate>,
    user_id: ObjectId,
}

impl FixRateAmount {
    pub fn new(old_rate: Option<Rate>, user_id: ObjectId) -> FixRateAmount {
        FixRateAmount { old_rate, user_id }
    }
}

#[async_trait]
impl View for FixRateAmount {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = "Введите сумму:";
        ctx.edit_origin(msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        if let Some(text) = msg.text() {
            if let Ok(amount) = text.parse::<Decimal>() {
                Ok(Jmp::Next(
                    FixRateDate::new(self.old_rate, self.user_id, amount).into(),
                ))
            } else {
                ctx.send_notification("Неверный формат суммы").await;
                Ok(Jmp::Stay)
            }
        } else {
            Ok(Jmp::Stay)
        }
    }
}

pub struct FixRateDate {
    amount: Decimal,
    old_rate: Option<Rate>,
    user_id: ObjectId,
}

impl FixRateDate {
    pub fn new(old_rate: Option<Rate>, user_id: ObjectId, amount: Decimal) -> FixRateDate {
        FixRateDate {
            old_rate,
            amount,
            user_id,
        }
    }
}

#[async_trait]
impl View for FixRateDate {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = "Введите дату следующего платежа: ДД\\.ММ\\.ГГГГ";
        ctx.edit_origin(msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;

        let text = msg.text().unwrap_or_default();
        let date = chrono::NaiveDate::parse_from_str(text, "%d.%m.%Y")
            .map_err(Error::new)
            .and_then(|date| {
                date.and_hms_opt(0, 0, 0)
                    .ok_or_else(|| eyre::eyre!("Invalid date"))
            })
            .and_then(|date| {
                Local
                    .from_local_datetime(&date)
                    .single()
                    .ok_or_else(|| eyre::eyre!("Invalid date"))
            });
        match date {
            Ok(date) => Ok(Jmp::Next(
                ConfirmCreationRate::new(
                    self.old_rate,
                    Rate::Fix {
                        amount: self.amount,
                        next_payment_date: date.with_timezone(&Utc),
                        reward_interval: Interval::Month { num: 1 },
                    },
                    self.user_id,
                )
                .into(),
            )),
            Err(_) => {
                ctx.send_notification("Введите дату в формате ДД\\.ММ\\.ГГГГ")
                    .await;
                Ok(Jmp::Stay)
            }
        }
    }
}
