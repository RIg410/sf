use std::mem;

use super::View;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{Jmp, ViewResult},
};
use bot_viewer::subscription::fmt_subscription_type;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use subscription::{
    model::{Subscription, SubscriptionType},
    service::CreateSubscriptionError,
};
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct CreateSubscription {
    state: State,
    subscription: Subscription,
}

impl Default for CreateSubscription {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateSubscription {
    pub fn new() -> CreateSubscription {
        CreateSubscription {
            state: State::SetName,
            subscription: Subscription::default(),
        }
    }

    async fn render_state(&self, ctx: &mut Context) -> Result<String> {
        let none = "❓".to_string();
        let (name, items, price, days, freeze, can_buy_by_user, sub_type, unlimited) =
            match self.state {
                State::SetName => (None, None, None, None, None, None, None, None),
                State::SetItems => (
                    Some(self.subscription.name.as_str()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                State::SetPrice => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                State::SetExpirationDaysDays => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                State::SetFreezeDays => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    None,
                    None,
                    None,
                    None,
                ),
                State::SetCanBuyWithUser => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    Some(self.subscription.freeze_days),
                    None,
                    None,
                    None,
                ),
                State::SubscriptionType => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    Some(self.subscription.freeze_days),
                    Some(self.subscription.user_can_buy),
                    None,
                    None,
                ),
                State::SubscriptionTypeFilter => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    Some(self.subscription.freeze_days),
                    Some(self.subscription.user_can_buy),
                    None,
                    None,
                ),
                State::SetUnlimited => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    Some(self.subscription.freeze_days),
                    Some(self.subscription.user_can_buy),
                    Some(self.subscription.subscription_type.clone()),
                    None,
                ),
                State::Finish => (
                    Some(self.subscription.name.as_str()),
                    Some(self.subscription.items),
                    Some(self.subscription.price),
                    Some(self.subscription.expiration_days),
                    Some(self.subscription.freeze_days),
                    Some(self.subscription.user_can_buy),
                    Some(self.subscription.subscription_type.clone()),
                    Some(self.subscription.unlimited),
                ),
            };

        let unlimited = unlimited.unwrap_or(false);

        if unlimited {
            Ok(format!(
                "📌 Тариф: *{}*\nЦена:*{}*\nСрок действия:*{}*\nЗаморозка:*{}*\nПользователь может купить:*{}*\n{}\n",
                escape(name.unwrap_or(&none)),
                price
                    .map(|i| i.to_string().replace(".", ","))
                    .unwrap_or_else(|| none.clone()),
                days.map(|i| i.to_string()).unwrap_or_else(|| none.clone()),
                freeze
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| none.clone()),
                can_buy_by_user
                    .map(|i| if i { "Да" } else { "Нет" }.to_string())
                    .unwrap_or_else(|| none.clone()),
                if let Some(sub_type) = sub_type {
                    fmt_subscription_type(ctx, &sub_type, false).await?
                } else {
                    none.clone()
                }
            ))
        } else {
            Ok(format!(
                "📌 Тариф: *{}*\nКоличество занятий:*{}*\nЦена:*{}*\nСрок действия:*{}*\nЗаморозка:*{}*\nПользователь может купить:*{}*\n{}\n",
                escape(name.unwrap_or(&none)),
                items.map(|i| i.to_string()).unwrap_or_else(|| none.clone()),
                price
                    .map(|i| i.to_string().replace(".", ","))
                    .unwrap_or_else(|| none.clone()),
                days.map(|i| i.to_string()).unwrap_or_else(|| none.clone()),
                freeze
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| none.clone()),
                can_buy_by_user
                    .map(|i| if i { "Да" } else { "Нет" }.to_string())
                    .unwrap_or_else(|| none.clone()),
                if let Some(sub_type) = sub_type {
                    fmt_subscription_type(ctx, &sub_type, false).await?
                } else {
                    none.clone()
                }
            ))
        }
    }
}

#[async_trait]
impl View for CreateSubscription {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut text = self.render_state(ctx).await?;
        text.push_str(&escape("-------------------\n"));
        let mut keymap = InlineKeyboardMarkup::default();
        match self.state {
            State::SetName => {
                text.push_str("*Введите название абонемента*");
            }
            State::SetItems => {
                text.push_str("*Введите количество занятий в абонементе*");
            }
            State::SetPrice => {
                text.push_str("*Введите стоимость абонемента*");
            }
            State::SetExpirationDaysDays => {
                text.push_str("*Введите срок действия абонемента\\(дни\\)*");
            }
            State::SetFreezeDays => {
                text.push_str("*Введите количество дней заморозки абонемента\\(дни\\)*");
            }
            State::SetCanBuyWithUser => {
                text.push_str("*Может ли пользователь купить этот абонемент?*");
                keymap = keymap.append_row(vec![
                    Callback::CanUserBuy(true).button("Да"),
                    Callback::CanUserBuy(false).button("Нет"),
                ]);
            }
            State::SubscriptionType => {
                text.push_str("*Выберите тип абонемента*");
                keymap = keymap.append_row(vec![
                    Callback::Group(true).button("Груповой"),
                    Callback::Group(false).button("Индивидуальный"),
                ]);
            }
            State::SetUnlimited => {
                text.push_str("*Безлимитный абонемент?*");
                keymap = keymap.append_row(vec![
                    Callback::Unlimited(true).button("Да"),
                    Callback::Unlimited(false).button("Нет"),
                ]);
            }
            State::Finish => {
                text.push_str("*Все верно?*");
                keymap = keymap.append_row(vec![
                    Callback::Create.button("✅ Сохранить"),
                    Callback::Cancel.button("❌ Отмена"),
                ]);
            }
            State::SubscriptionTypeFilter => {
                text.push_str("*Выберите инструктора*");
                let couch_list = ctx.services.users.instructors(&mut ctx.session).await?;
                for couch in couch_list {
                    keymap = keymap.append_row(vec![
                        Callback::Couch(couch.id.bytes()).button(&couch.name.first_name),
                    ]);
                }
            }
        }

        ctx.send_msg_with_markup(&text, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        let text = if let Some(text) = message.text() {
            text
        } else {
            return Ok(Jmp::Stay);
        };

        self.state = match self.state {
            State::SetName => {
                let name = text.to_string();
                let sub = ctx
                    .services
                    .subscriptions
                    .get_by_name(&mut ctx.session, &name)
                    .await?;
                if sub.is_some() {
                    ctx.send_msg("Абонемент с таким именем уже существует")
                        .await?;
                    return Ok(Jmp::Stay);
                }
                self.subscription.name = text.to_string();
                State::SetItems
            }
            State::SetItems => {
                if let Ok(items) = text.parse() {
                    self.subscription.items = items;
                    State::SetPrice
                } else {
                    ctx.send_msg("Введите число").await?;
                    State::SetItems
                }
            }
            State::SetPrice => {
                if let Ok(price) = text.parse() {
                    self.subscription.price = price;
                    State::SetExpirationDaysDays
                } else {
                    ctx.send_msg("Введите число").await?;
                    State::SetPrice
                }
            }
            State::SetExpirationDaysDays => {
                if let Ok(expiration_days) = text.parse() {
                    self.subscription.expiration_days = expiration_days;
                    State::SetFreezeDays
                } else {
                    ctx.send_msg("Введите число").await?;
                    State::SetExpirationDaysDays
                }
            }
            State::SetFreezeDays => {
                if let Ok(freeze_days) = text.parse() {
                    self.subscription.freeze_days = freeze_days;
                    State::SetCanBuyWithUser
                } else {
                    ctx.send_msg("Введите число").await?;
                    State::SetFreezeDays
                }
            }
            State::SetCanBuyWithUser => {
                ctx.delete_msg(message.id).await?;
                State::SetCanBuyWithUser
            }
            State::SubscriptionType => {
                ctx.delete_msg(message.id).await?;
                State::SubscriptionType
            }
            State::SubscriptionTypeFilter => {
                ctx.delete_msg(message.id).await?;
                State::SubscriptionTypeFilter
            }
            State::SetUnlimited => {
                ctx.delete_msg(message.id).await?;
                State::SetUnlimited
            }
            State::Finish => {
                ctx.delete_msg(message.id).await?;
                State::Finish
            }
        };

        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Create => {
                ctx.ensure(Rule::CreateSubscription)?;
                let sub = mem::take(&mut self.subscription);
                let result = ctx
                    .services
                    .subscriptions
                    .create_subscription(&mut ctx.session, sub)
                    .await;
                match result {
                    Ok(_) => {
                        ctx.send_msg("✅Абонемент создан").await?;
                        ctx.reset_origin();
                        Ok(Jmp::Back(1))
                    }
                    Err(CreateSubscriptionError::NameAlreadyExists) => {
                        ctx.send_msg("Не удалось создать абонемент: Имя уже занято")
                            .await?;
                        Ok(Jmp::Stay)
                    }
                    Err(CreateSubscriptionError::InvalidPrice) => {
                        ctx.send_msg("Не удалось создать абонемент: Неверная цена")
                            .await?;
                        Ok(Jmp::Stay)
                    }
                    Err(CreateSubscriptionError::InvalidItems) => {
                        ctx.send_msg("Не удалось создать абонемент: Неверное количество занятий")
                            .await?;
                        Ok(Jmp::Stay)
                    }
                    Err(CreateSubscriptionError::Common(err)) => Err(err.into()),
                }
            }
            Callback::Cancel => Ok(Jmp::Back(1)),
            Callback::CanUserBuy(can_by) => {
                if self.state == State::SetCanBuyWithUser {
                    self.subscription.user_can_buy = can_by;
                    self.state = State::SubscriptionType;
                }
                Ok(Jmp::Stay)
            }
            Callback::Group(is_group) => {
                if is_group {
                    self.subscription.subscription_type = SubscriptionType::Group {
                        program_filter: vec![],
                    };
                    self.state = State::SetUnlimited;
                } else {
                    self.subscription.subscription_type = SubscriptionType::Personal {
                        couch_filter: ObjectId::from_bytes(Default::default()),
                    };
                    self.state = State::SubscriptionTypeFilter;
                }
                Ok(Jmp::Stay)
            }
            Callback::Unlimited(unlimited) => {
                self.subscription.unlimited = unlimited;
                self.state = State::Finish;
                Ok(Jmp::Stay)
            }
            Callback::Couch(couch_id) => {
                self.subscription.subscription_type = SubscriptionType::Personal {
                    couch_filter: ObjectId::from_bytes(couch_id),
                };
                self.state = State::SetUnlimited;
                Ok(Jmp::Stay)
            }
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    SetName,
    SetItems,
    SetPrice,
    SetExpirationDaysDays,
    SetFreezeDays,
    SetCanBuyWithUser,
    SubscriptionType,
    SubscriptionTypeFilter,
    SetUnlimited,
    Finish,
}

#[derive(Serialize, Deserialize)]
enum Callback {
    Create,
    Cancel,
    CanUserBuy(bool),
    Group(bool),
    Couch([u8; 12]),
    Unlimited(bool),
}
