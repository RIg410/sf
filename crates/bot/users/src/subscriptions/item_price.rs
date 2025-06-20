use std::str::FromStr as _;

use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use decimal::Decimal;
use eyre::Error;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct SetItemPrice {
    user_id: ObjectId,
    id: ObjectId,
}

impl SetItemPrice {
    pub fn new(user_id: ObjectId, id: ObjectId) -> Self {
        Self { user_id, id }
    }
}

#[async_trait]
impl View for SetItemPrice {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), Error> {
        ctx.ensure(Rule::EditSubscription)?;
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.user_id)
            .await?;
        let user = user.payer()?;
        let subs = user.subscriptions();
        let sub = subs
            .iter()
            .find(|s| s.id == self.id)
            .ok_or_else(|| eyre::eyre!("Subscription not found"))?;

        let msg = format!(
            "*Выберите цену занятия*\nТекущая цена занятия: {}",
            escape(&sub.item_price().to_string())
        );

        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        if let Some(price) = msg.text() {
            let price = Decimal::from_str(price)?;
            ctx.services
                .users
                .set_subscription_item_price(&mut ctx.session, self.user_id, self.id, price)
                .await?;
        } else {
            ctx.send_msg("Введите цену занятия").await?;
        }

        Ok(Jmp::Stay)
    }
}
