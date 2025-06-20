use crate::edit_programs::EditPrograms;

use super::{
    View,
    edit::{EditSubscription, EditType},
    sell::SellView,
};
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, ViewResult},
};
use bot_viewer::subscription::fmt_subscription_type;
use eyre::{Context as _, Error, Result};
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

pub struct SubscriptionOption {
    id: ObjectId,
}

impl SubscriptionOption {
    pub fn new(id: ObjectId) -> SubscriptionOption {
        SubscriptionOption { id }
    }

    async fn edit(&mut self, tp: EditType) -> ViewResult {
        Ok(EditSubscription::new(self.id, tp).into())
    }

    async fn edit_requirement(&mut self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::EditSubscription)?;
        Ok(EditPrograms::new(self.id).into())
    }

    async fn buy(&mut self, ctx: &mut Context) -> ViewResult {
        let sub = ctx
            .services
            .subscriptions
            .get(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Subscription not found"))?;
        if !sub.user_can_buy {
            ctx.send_msg("Покупка абонемента недоступна").await?;
            return Ok(Jmp::Back(1));
        }

        Ok(Jmp::Back(1))
    }
}

#[async_trait]
impl View for SubscriptionOption {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let (txt, keymap) = render_sub(self.id, ctx).await.context("render")?;
        ctx.edit_origin(&txt, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Delete => {
                ctx.ensure(Rule::EditSubscription)?;
                ctx.services
                    .subscriptions
                    .delete(&mut ctx.session, self.id)
                    .await?;
                Ok(Jmp::Back(1))
            }
            Callback::Buy => {
                ctx.ensure(Rule::BuySubscription)?;
                self.buy(ctx).await
            }
            Callback::Sell => {
                ctx.ensure(Rule::SellSubscription)?;
                Ok(SellView::new(self.id).into())
            }
            Callback::EditPrice => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::Price).await
            }
            Callback::EditPrograms => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit_requirement(ctx).await
            }
            Callback::EditItems => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::Items).await
            }
            Callback::EditName => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::Name).await
            }
            Callback::EditFreezeDays => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::FreezeDays).await
            }
            Callback::EditCanBuyByUser => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::CanBuyByUser).await
            }
            Callback::EditExpirationDays => {
                ctx.ensure(Rule::EditSubscription)?;
                self.edit(EditType::ExpirationDays).await
            }
        }
    }
}

async fn render_sub(
    id: ObjectId,
    ctx: &mut Context,
) -> Result<(String, InlineKeyboardMarkup), Error> {
    let sub = ctx
        .services
        .subscriptions
        .get(&mut ctx.session, id)
        .await?
        .ok_or_else(|| eyre::eyre!("Subscription not found"))?;

    let msg = if sub.unlimited {
        format!(
            "📌 Тариф: _{}_\nБезлимитный\nЦена:_{}_\nДни заморозки:_{}_\nДействует дней:_{}_\n{}\n",
            escape(&sub.name),
            sub.price.to_string().replace(".", ","),
            sub.freeze_days,
            sub.expiration_days,
            fmt_subscription_type(
                ctx,
                &sub.subscription_type,
                !ctx.has_right(Rule::EditSubscription)
            )
            .await?,
        )
    } else {
        format!(
            "📌 Тариф: _{}_\nКоличество занятий:_{}_\nЦена:_{}_\nДни заморозки:_{}_\nДействует дней:_{}_\n{}\n",
            escape(&sub.name),
            sub.items,
            sub.price.to_string().replace(".", ","),
            sub.freeze_days,
            sub.expiration_days,
            fmt_subscription_type(
                ctx,
                &sub.subscription_type,
                !ctx.has_right(Rule::EditSubscription)
            )
            .await?,
        )
    };

    let mut keymap = InlineKeyboardMarkup::default();

    if ctx.has_right(Rule::BuySubscription) {
        keymap = keymap.append_row(Callback::Buy.btn_row("🛒 Купить"));
    }
    if ctx.has_right(Rule::SellSubscription) {
        keymap = keymap.append_row(Callback::Sell.btn_row("🛒 Продать"));
    }
    if ctx.has_right(Rule::EditSubscription) {
        keymap = keymap.append_row(Callback::Delete.btn_row("❌ Удалить"));
        keymap = keymap.append_row(Callback::EditPrice.btn_row("Изменить цену 💸"));
        keymap = keymap.append_row(Callback::EditItems.btn_row("Изменить количество занятий"));
        keymap = keymap.append_row(Callback::EditName.btn_row("Изменить название"));
        keymap = keymap.append_row(Callback::EditFreezeDays.btn_row("Изменить дни заморозки"));
        keymap = keymap
            .append_row(Callback::EditCanBuyByUser.btn_row("Изменить доступность для покупки"));
        keymap = keymap.append_row(Callback::EditExpirationDays.btn_row("Изменить время действия"));
        if sub.subscription_type.is_group() {
            keymap = keymap.append_row(Callback::EditPrograms.btn_row("Изменить программы"));
        }
    }

    Ok((msg, keymap))
}

#[derive(Serialize, Deserialize)]
enum Callback {
    Delete,
    Sell,
    Buy,
    EditPrice,
    EditItems,
    EditPrograms,
    EditName,
    EditFreezeDays,
    EditCanBuyByUser,
    EditExpirationDays,
}
