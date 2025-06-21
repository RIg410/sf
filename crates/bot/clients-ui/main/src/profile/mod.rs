use async_trait::async_trait;
use bot_client_trainings::list::TrainingList;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    views::ask::AskViewWidget,
    widget::{View, ViewResult},
};
use bot_viewer::{fmt_phone, user::fmt_user_type};
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

use crate::{
    profile::{
        freeze::{AskFreezeDays, UnfreezeConfirm},
        history::HistoryList,
    },
    render_freeze_info, render_subscriptions, render_trainings,
};

pub mod freeze;
mod history;

pub struct ProfileView;

#[async_trait]
impl View for ProfileView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let empty = "?".to_string();

        let extension = ctx
            .services
            .users
            .get_extension(&mut ctx.session, ctx.me.id)
            .await?;

        let mut keymap = InlineKeyboardMarkup::default();

        let mut msg = format!(
            "{} Пользователь : _{}_
*{}* _{}_
Телефон : {}
Дата рождения : _{}_\n",
            fmt_user_type(&ctx.me),
            escape(
                &ctx.me
                    .name
                    .tg_user_name
                    .as_ref()
                    .map(|n| format!("@{n}"))
                    .unwrap_or_else(|| empty.to_owned())
            ),
            escape(&ctx.me.name.first_name),
            escape(ctx.me.name.last_name.as_ref().unwrap_or(&empty)),
            fmt_phone(ctx.me.phone.as_deref()),
            escape(
                &extension
                    .birthday
                    .as_ref()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| empty.clone())
            ),
        );

        render_freeze_info(ctx, &mut msg)?;
        render_subscriptions(ctx, &mut msg)?;
        render_family(ctx, &mut msg)?;
        render_trainings(ctx, &mut msg, 10).await?;

        let client = ctx.me.as_client()?;

        if client.freeze_days != 0 && client.freeze.is_none() {
            keymap = keymap.append_row(Callback::Freeze.btn_row("Заморозить ❄"));
        } else if client.freeze.is_some() {
            keymap = keymap.append_row(Callback::UnFreeze.btn_row("Разморозить ❄"));
        }

        keymap = keymap.append_row(Callback::TrainingList.btn_row("Записи 📝"));
        keymap = keymap.append_row(Callback::HistoryList.btn_row("История 📝"));

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::UnFreeze => Ok(UnfreezeConfirm.into()),
            Callback::Freeze => Ok(AskViewWidget::new(AskFreezeDays).into()),
            Callback::TrainingList => Ok(TrainingList::client(ctx.me.id).into()),
            Callback::HistoryList => Ok(HistoryList::default().into()),
        }
    }
}

fn render_family(ctx: &mut Context, msg: &mut String) -> eyre::Result<()> {
    let family = &ctx.me.family;
    if let Some(payer) = family.payer.as_ref() {
        msg.push_str(&format!(
            "Глава семьи: *{}*\n",
            escape(&payer.name.first_name)
        ));
    }

    if !family.children.is_empty() {
        msg.push_str("Члены семьи:\n");
        for child in family.children.iter() {
            msg.push_str(&format!(
                "👤 *{}* {}\n",
                escape(&child.name.first_name),
                if child.family.is_individual {
                    "Независимый"
                } else {
                    "Общие абонементы"
                }
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Callback {
    Freeze,
    UnFreeze,
    TrainingList,
    HistoryList,
}
