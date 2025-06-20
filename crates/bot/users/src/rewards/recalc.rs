use async_trait::async_trait;
use bot_core::callback_data::Calldata as _;
use bot_core::calldata;
use bot_core::widget::{Jmp, ViewResult};
use bot_core::{context::Context, widget::View};
use decimal::Decimal;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

use crate::profile::UserProfile;

pub struct AddRecalcReward {
    pub user_id: ObjectId,
}

impl AddRecalcReward {
    pub fn new(user_id: ObjectId) -> Self {
        AddRecalcReward { user_id }
    }
}

#[async_trait]
impl View for AddRecalcReward {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.ensure(Rule::RecalculateRewards)?;
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.user_id)
            .await?;

        let msg = format!(
            "Пересчет награды для пользователя *{}*\n\nВведите сумму коррекции:",
            escape(&user.name.first_name)
        );
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.bot.delete_msg(message.id).await?;
        let text = message
            .text()
            .ok_or_else(|| eyre::eyre!("Сообщение не является текстом"))?;

        let amount = text
            .parse::<Decimal>()
            .map_err(|_| eyre::eyre!("Сумма коррекции должна быть числом"))?;
        Ok(Jmp::Next(
            AddRecalcComment::new(self.user_id, amount).into(),
        ))
    }
}

pub struct AddRecalcComment {
    pub user_id: ObjectId,
    pub amount: Decimal,
}

impl AddRecalcComment {
    pub fn new(user_id: ObjectId, amount: Decimal) -> Self {
        AddRecalcComment { user_id, amount }
    }
}

#[async_trait]
impl View for AddRecalcComment {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.ensure(Rule::RecalculateRewards)?;
        let msg = "Введите комментарий к коррекции:".to_string();
        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.bot.delete_msg(message.id).await?;
        let text = message
            .text()
            .ok_or_else(|| eyre::eyre!("Сообщение не является текстом"))?;

        let comment = text.to_string();

        Ok(Jmp::Next(
            AddRecalcConfirm {
                user_id: self.user_id,
                amount: self.amount,
                comment,
            }
            .into(),
        ))
    }
}

pub struct AddRecalcConfirm {
    pub user_id: ObjectId,
    pub amount: Decimal,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Callback {
    Yes,
    No,
}

#[async_trait]
impl View for AddRecalcConfirm {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.ensure(Rule::RecalculateRewards)?;
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.user_id)
            .await?;
        let msg = format!(
            "Подтверждение коррекции награды для пользователя *{}*\n\nСумма: *{}*\nКомментарий: *{}*",
            escape(&user.name.first_name),
            escape(&self.amount.to_string()),
            escape(&self.comment)
        );

        let keymap = vec![vec![
            Callback::Yes.button("✅ Да. Подтверждаю"),
            Callback::No.button("❌ Отмена"),
        ]];
        ctx.edit_origin(&msg, InlineKeyboardMarkup::new(keymap))
            .await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::RecalculateRewards)?;

        match calldata!(data) {
            Callback::Yes => {
                ctx.services
                    .employee
                    .add_recalculation_reward(
                        &mut ctx.session,
                        self.user_id,
                        self.amount,
                        self.comment.clone(),
                    )
                    .await?;
            }
            Callback::No => {
                // no-op
            }
        }
        Ok(Jmp::Goto(UserProfile::new(self.user_id).into()))
    }
}
