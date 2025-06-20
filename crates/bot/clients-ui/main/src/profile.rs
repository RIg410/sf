use async_trait::async_trait;
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
    freeze::{AskFreezeDays, UnfreezeConfirm},
    history::HistoryList,
    render_freeze_info, render_subscriptions, render_trainings,
};

pub struct ProfileView;

impl ProfileView {
    // async fn training_list(&mut self, ctx: &mut Context) -> ViewResult {
    //     let user = ctx
    //         .services
    //         .users
    //         .get_user(&mut ctx.session, self.id)
    //         .await?;
    //     if user.employee.is_some() {
    //         Ok(TrainingList::couches(user.id).into())
    //     } else {
    //         Ok(TrainingList::users(user.id).into())
    //     }
    // }

    // async fn history_list(&mut self, ctx: &mut Context) -> ViewResult {
    //     let user = ctx
    //         .services
    //         .users
    //         .get_user(&mut ctx.session, self.id)
    //         .await?;
    //     Ok(HistoryList::new(user.id).into())
    // }

    // async fn family_view(&mut self, ctx: &mut Context, id: ObjectId) -> ViewResult {
    //     if ctx.has_right(Rule::ViewFamily) || (ctx.me.id == id && ctx.me.has_family()) {
    //         Ok(FamilyView::new(self.id).into())
    //     } else {
    //         Ok(Jmp::Stay)
    //     }
    // }
}

#[async_trait]
impl View for ProfileView {
    fn name(&self) -> &'static str {
        "UserProfile"
    }

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
        render_trainings(ctx, &mut msg, 10).await?;

        if ctx.me.freeze_days != 0 && ctx.me.freeze.is_none() {
            keymap = keymap.append_row(Callback::Freeze.btn_row("Заморозить ❄"));
        } else if ctx.me.freeze.is_some() {
            keymap = keymap.append_row(Callback::UnFreeze.btn_row("Разморозить ❄"));
        }

        if ctx.me.has_family() {
            keymap = keymap.append_row(Callback::FamilyView.btn_row("Семья 👨‍👩‍👧‍👦"));
        }

        keymap = keymap.append_row(Callback::TrainingList.btn_row("Записи 📝"));
        keymap = keymap.append_row(Callback::HistoryList.btn_row("История 📝"));

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::UnFreeze => Ok(UnfreezeConfirm.into()),
            Callback::Freeze => Ok(AskViewWidget::new(AskFreezeDays).into()),
            Callback::TrainingList => todo!(),
            Callback::HistoryList => Ok(HistoryList::default().into()),
            Callback::FamilyView => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Callback {
    Freeze,
    UnFreeze,
    TrainingList,
    HistoryList,
    FamilyView,
}
