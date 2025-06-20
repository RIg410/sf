use crate::profile::{freeze::UnfreezeConfirm, ProfileView};
use bot_core::callback_data::Calldata;
use bot_core::calldata;
use bot_core::context::Context;
use bot_core::widget::ViewResult;
use bot_viewer::day::fmt_date;
use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use subscription::model::{SubscriptionStatus, UserSubscription};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};
use users::model::role::UserRole;

pub mod profile;

#[derive(Default)]
pub struct ClientMain;

impl ClientMain {}

#[async_trait::async_trait]
impl bot_core::widget::View for ClientMain {
    fn name(&self) -> &'static str {
        "ClientMain"
    }

    fn safe_point(&self) -> bool {
        true
    }

    fn main_view(&self) -> bool {
        true
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let mut markup = InlineKeyboardMarkup::default();

        let _info = if let UserRole::Client(info) = &ctx.me.role {
            info
        } else {
            return Err(eyre::eyre!("User is not a client"));
        };

        let mut msg = format!("Привет, {}\\! 👋\n\n", escape(&ctx.me.name.first_name));

        render_freeze_info(ctx, &mut msg)?;

        if ctx.me.freeze_days > 0 {
            msg.push_str(&format!(
                "Доступно дней заморозки: *{}*\\.\n\n",
                ctx.me.freeze_days
            ));
        }
        render_subscriptions(ctx, &mut msg)?;
        render_trainings(ctx, &mut msg, 5).await?;

        markup = markup
            .append_row(Callback::Schedule.btn_row("📅 Расписание"))
            .append_row(Callback::Programs.btn_row("📋 Программы"))
            .append_row(Callback::Couches.btn_row("🧘 Инструкторы"))
            .append_row(Callback::Shop.btn_row("🛒 Магазин"))
            .append_row(Callback::Profile.btn_row("👤 Профиль"));

        if ctx.me.freeze.is_some() {
            markup = markup.append_row(Callback::Unfreeze.btn_row("❄️ Разморозить"));
        }

        ctx.edit_origin(&msg, markup).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Schedule => todo!(),
            Callback::Programs => todo!(),
            Callback::Couches => todo!(),
            Callback::Shop => todo!(),
            Callback::Profile => Ok(ProfileView.into()),
            Callback::Unfreeze => Ok(UnfreezeConfirm.into()),
        }
    }
}

pub fn render_freeze_info(ctx: &mut Context, msg: &mut String) -> eyre::Result<()> {
    if let Some(freeze) = ctx.me.freeze.as_ref() {
        msg.push_str(&format!(
            "❄️ Заморожен c _{}_  по _{}_",
            fmt_date(&freeze.freeze_start.with_timezone(&Local)),
            fmt_date(&freeze.freeze_end.with_timezone(&Local))
        ));
    }
    Ok(())
}

pub fn render_sub(sub: &UserSubscription, is_owner: bool) -> String {
    let now = Utc::now();

    let emoji = if is_owner { "💳" } else { "🎟" };

    match sub.status {
        SubscriptionStatus::NotActive => {
            if sub.unlimited {
                format!(
                    "{}_{}_\nБезлимитный абонемент\nНе активен\\. \n",
                    emoji,
                    escape(&sub.name),
                )
            } else {
                format!(
                    "{}_{}_\nОсталось занятий:*{}*\\(_{}_ резерв\\)\n   Не активен\\. \n",
                    emoji,
                    escape(&sub.name),
                    sub.balance,
                    sub.locked_balance,
                )
            }
        }
        SubscriptionStatus::Active {
            start_date,
            end_date,
        } => {
            let exp = if sub.is_expired(now) {
                "\n*Абонемент истек*"
            } else {
                ""
            };
            if sub.unlimited {
                format!(
                    "{}_{}_\nБезлимитный абонемент\nДействует c _{}_ по _{}_{}",
                    emoji,
                    escape(&sub.name),
                    start_date.with_timezone(&Local).format("%d\\.%m\\.%Y"),
                    end_date.with_timezone(&Local).format("%d\\.%m\\.%Y"),
                    exp
                )
            } else {
                format!(
                    "{}_{}_\nОсталось занятий:*{}*\\(_{}_ резерв\\)\nДействует c _{}_ по _{}_{}",
                    emoji,
                    escape(&sub.name),
                    sub.balance,
                    sub.locked_balance,
                    start_date.with_timezone(&Local).format("%d\\.%m\\.%Y"),
                    end_date.with_timezone(&Local).format("%d\\.%m\\.%Y"),
                    exp
                )
            }
        }
    }
}

pub async fn render_trainings(
    ctx: &mut Context,
    msg: &mut String,
    limit: usize,
) -> eyre::Result<()> {
    let trainings = ctx
        .services
        .calendar
        .find_trainings(
            &mut ctx.session,
            trainings::model::Filter::Client(ctx.me.id),
            limit,
            0,
        )
        .await?;
    if !trainings.is_empty() {
        msg.push_str("\nЗаписи:\n");
        for training in trainings {
            msg.push_str(&escape(&format!(
                "{} {}\n",
                training.get_slot().start_at().format("%d.%m %H:%M"),
                training.name
            )))
        }
        msg.push_str("➖➖➖➖➖➖➖➖➖➖\n");
    }
    Ok(())
}

pub fn render_subscriptions(ctx: &mut Context, msg: &mut String) -> eyre::Result<()> {
    let payer = ctx.me.payer()?;
    let mut subs = payer.subscriptions().to_vec();
    subs.sort_by(|a, b| a.status.cmp(&b.status));

    if !subs.is_empty() {
        msg.push_str("Абонементы:\n");
    }

    let has_group = subs.iter().any(|s| !s.tp.is_personal());
    let has_personal = subs.iter().any(|s| s.tp.is_personal());

    if has_group {
        msg.push_str("Групповые:");
        for sub in &subs {
            if sub.tp.is_personal() {
                continue;
            }
            msg.push('\n');
            msg.push_str(&render_sub(sub, payer.is_owner()));
        }
        msg.push_str("➖➖➖➖➖➖➖➖➖➖\n");
    }

    if has_personal {
        msg.push_str("Персональные:");

        for sub in &subs {
            if !sub.tp.is_personal() {
                continue;
            }
            msg.push('\n');
            msg.push_str(&render_sub(sub, payer.is_owner()));
        }
    }
    if subs.is_empty() {
        msg.push_str("*нет абонементов*🥺\n");
    }
    msg.push_str("➖➖➖➖➖➖➖➖➖➖\n");
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Callback {
    Schedule,
    Programs,
    Couches,
    Shop,
    Profile,
    Unfreeze,
}
