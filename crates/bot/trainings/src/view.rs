use crate::{client::list::ClientsList, edit::EditTraining, family::FamilySignIn};
use async_trait::async_trait;
use booking::payer::{FindFor, SubscriptionResolver as _};
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    views::done::DoneView,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::{day::fmt_dt, fmt_phone, training::fmt_training_type};
use chrono::Local;
use eyre::{Result, eyre};
use ident::training::TrainingId;
use mongodb::bson::oid::ObjectId;
use program::model::TrainingType;
use rights::Rule;
use serde::{Deserialize, Serialize};
use std::vec;
use teloxide::{
    types::{ChatId, InlineKeyboardMarkup},
    utils::markdown::escape,
};
use trainings::model::{Training, status::TrainingStatus};

pub struct TrainingView {
    id: TrainingId,
}

impl TrainingView {
    pub fn new(id: TrainingId) -> Self {
        Self { id }
    }

    async fn couch_info(&mut self, ctx: &mut Context) -> ViewResult {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, training.instructor)
            .await?;
        if let Some(couch) = user.employee {
            ctx.send_msg(&escape(&couch.description)).await?;
        }
        Ok(Jmp::Stay)
    }

    async fn restore_training(&mut self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::CancelTraining)?;
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        if !training.is_group() {
            return Err(eyre!("Can't delete personal training").into());
        }

        ctx.services
            .booking
            .restore_training(&mut ctx.session, &training)
            .await?;
        Ok(Jmp::Stay)
    }

    async fn client_list(&mut self, ctx: &mut Context) -> ViewResult {
        if !ctx.is_employee() && !ctx.has_right(Rule::EditTrainingClientsList) {
            return Err(eyre!("Only couch can see client list").into());
        }
        Ok(ClientsList::new(self.id).into())
    }
}

#[async_trait]
impl View for TrainingView {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        let (msg, keymap) = render(ctx, &training).await?;
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::CouchInfo => self.couch_info(ctx).await,
            Callback::Cancel => return Ok(Jmp::Next(ConfirmCancelTraining::new(self.id).into())),
            Callback::UnCancel => self.restore_training(ctx).await,
            Callback::SignUp => sign_up(ctx, self.id, ctx.me.id).await,
            Callback::SignOut => sign_out(ctx, self.id, ctx.me.id).await,
            Callback::ClientList => self.client_list(ctx).await,
            Callback::OpenSignInView => Ok(Jmp::Next(FamilySignIn::new(self.id).into())),
            Callback::Edit => Ok(EditTraining::new(self.id).into()),
        }
    }
}

async fn render(ctx: &mut Context, training: &Training) -> Result<(String, InlineKeyboardMarkup)> {
    let is_client = ctx.me.employee.is_none();
    let cap = if is_client {
        format!(
            "*Свободных мест*: _{}_",
            training
                .capacity
                .saturating_sub(training.clients.len() as u32)
        )
    } else {
        format!(
            "*Места* :_{}/{}_",
            training.clients.len(),
            training.capacity
        )
    };

    let now = Local::now();
    let tr_status = training.status(now);
    let slot = training.get_slot();
    let signed = training.clients.contains(&ctx.me.id);

    let couch = ctx
        .services
        .users
        .get(&mut ctx.session, training.instructor)
        .await?
        .map(|couch| {
            format!(
                "_{}_ {}",
                escape(&couch.name.first_name),
                escape(&couch.name.last_name.unwrap_or_default())
            )
        })
        .unwrap_or_default();

    let tp = match training.tp {
        TrainingType::Group { .. } => "групповая тренировка",
        TrainingType::Personal { .. } => "персональная тренировка",
        TrainingType::SubRent { .. } => "аренда",
    };

    let msg = format!(
        "
💪 *{}*: _{}_
📅 *Дата*: _{}_
🧘 *Инструктор*: {}
💁{}
⏱*Продолжительность*: _{}_мин
_{}_                            \n
[Описание]({})
{}
{}
",
        tp,
        escape(&training.name),
        fmt_dt(&slot.start_at()),
        couch,
        cap,
        training.duration_min,
        status(tr_status, training.is_full()),
        training.description,
        fmt_training_type(training.tp),
        if signed {
            "❤️ Вы записаны"
        } else {
            ""
        }
    );

    let mut keymap = InlineKeyboardMarkup::default();
    if training.is_group() || training.is_personal() {
        keymap = keymap.append_row(vec![Callback::CouchInfo.button("🧘 Об инструкторе")]);
    }

    if ctx.has_right(Rule::EditTrainingClientsList) && training.is_group() || training.is_personal()
    {
        keymap = keymap.append_row(vec![Callback::ClientList.button("🗒 Список клиентов")]);
    }

    let mut row = vec![];
    if ctx.has_right(Rule::CancelTraining) || ctx.me.id == training.instructor {
        if tr_status.can_be_canceled() {
            row.push(Callback::Cancel.button("⛔ Отменить"));
        }
        if tr_status.can_be_uncanceled() {
            row.push(Callback::UnCancel.button("🔓 Вернуть"));
        }
    }
    keymap = keymap.append_row(row);

    if training.is_group() {
        if !EditTraining::hidden(ctx)? && !training.is_processed {
            keymap = keymap.append_row(vec![Callback::Edit.button("🔄 Редактировать")]);
        }

        if is_client {
            if ctx.me.family.children_ids.is_empty() {
                if signed {
                    if tr_status.can_sign_out() {
                        keymap =
                            keymap.append_row(vec![Callback::SignOut.button("❌ Отменить запись")]);
                    }
                } else if tr_status.can_sign_in() {
                    keymap = keymap.append_row(vec![Callback::SignUp.button("✔️ Записаться")]);
                }
            } else {
                keymap = keymap.append_row(vec![Callback::OpenSignInView.button("👨‍👩‍👧‍👦 Запись")]);
            }
        }
    }

    if training.is_personal() && signed {
        keymap = keymap.append_row(vec![Callback::SignOut.button("❌ Отменить запись")]);
    }

    Ok((msg, keymap))
}

#[derive(Serialize, Deserialize)]
enum Callback {
    CouchInfo,
    Cancel,
    ClientList,
    UnCancel,
    SignUp,
    SignOut,
    OpenSignInView,
    Edit,
}

fn status(status: TrainingStatus, is_full: bool) -> &'static str {
    match status {
        TrainingStatus::OpenToSignup { .. } => {
            if is_full {
                "нет мест ✌️"
            } else {
                "🟢Открыта для записи"
            }
        }
        TrainingStatus::ClosedToSignup => "🟠Запись закрыта",
        TrainingStatus::InProgress => "🤸🏼 Идет",
        TrainingStatus::Cancelled => "⛔Отменена",
        TrainingStatus::Finished => "✔️Завершена",
    }
}
pub struct ConfirmCancelTraining {
    id: TrainingId,
}

impl ConfirmCancelTraining {
    pub fn new(id: TrainingId) -> Self {
        Self { id }
    }

    async fn cancel_training(&mut self, ctx: &mut Context) -> Result<Jmp> {
        ctx.ensure(Rule::CancelTraining)?;
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        let to_notify = ctx
            .services
            .booking
            .cancel_training(&mut ctx.session, &training)
            .await?;
        let msg = format!(
            "Тренировка '{}' в {} *отменена*\\.",
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        );
        for client in to_notify {
            if let Ok(user) = ctx.services.users.get_user(&mut ctx.session, client).await {
                ctx.bot.notify(ChatId(user.tg_id), &msg, true).await;
            }
        }
        if training.is_group() {
            Ok(Jmp::ToSafePoint)
        } else {
            Ok(Jmp::ToSafePoint)
        }
    }
}

#[async_trait]
impl View for ConfirmCancelTraining {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        let tp = match training.tp {
            TrainingType::Group { .. } => "групповую тренировку",
            TrainingType::Personal { .. } => "персональную тренировку",
            TrainingType::SubRent { .. } => "аренду",
        };

        let msg = format!(
            "Вы уверены, что хотите отменить {} '{}' в {}?",
            tp,
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        );
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(vec![
            CancelCallback::Cancel.button("✅ Да"),
            CancelCallback::Stay.button("❌ нет"),
        ]);
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        Ok(match calldata!(data) {
            CancelCallback::Cancel => self.cancel_training(ctx).await?,
            CancelCallback::Stay => Jmp::Back(1),
        })
    }
}

#[derive(Serialize, Deserialize)]
enum CancelCallback {
    Cancel,
    Stay,
}

pub async fn sign_up(ctx: &mut Context, id: TrainingId, user_id: ObjectId) -> ViewResult {
    let training = ctx
        .services
        .calendar
        .get_training_by_id(&mut ctx.session, id)
        .await?
        .ok_or_else(|| eyre::eyre!("Training not found"))?;
    if !training.status(Local::now()).can_sign_in() {
        return Ok(DoneView::err(format!(
            "Запись на тренировку *{}* в {} *закрыта*💔",
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        ))
        .go_back()
        .into());
    }
    if !training.is_group() {
        return Err(eyre!("Can't delete personal training").into());
    }
    if training.is_full() {
        return Ok(DoneView::err(format!(
            "На тренировку *{}* в {} *нет мест*🥺",
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        ))
        .go_back()
        .into());
    }

    let mut user = ctx
        .services
        .users
        .get_user(&mut ctx.session, user_id)
        .await?;

    if training.tp.is_not_free() {
        let mut payer = user.payer_mut()?;
        if let Some(sub) = payer.find_subscription(FindFor::Lock, &training) {
            if sub.balance < 1 {
                return Ok(DoneView::err("В абонементе нет занятий🥺").go_back().into());
            }
        } else {
            return Ok(DoneView::err("Нет подходящего абонемента🥺")
                .go_back()
                .into());
        };
    }

    if let Some(freeze) = ctx.me.as_client()?.freeze.as_ref() {
        let slot = training.get_slot();
        if freeze.freeze_start <= slot.start_at() && freeze.freeze_end >= slot.end_at() {
            return Ok(DoneView::err("Ваш абонемент заморожен🥶").go_back().into());
        }
        return Ok(Jmp::Stay);
    }

    ctx.services
        .booking
        .sign_up(&mut ctx.session, id, user.id, false)
        .await?;

    let msg = format!(
        "Вы записаны на тренировку *{}* в {}",
        escape(&training.name),
        fmt_dt(&training.get_slot().start_at())
    );
    Ok(DoneView::ok(msg).into())
}

pub async fn sign_out(ctx: &mut Context, id: TrainingId, user_id: ObjectId) -> ViewResult {
    let training = ctx
        .services
        .calendar
        .get_training_by_id(&mut ctx.session, id)
        .await?
        .ok_or_else(|| eyre::eyre!("Training not found"))?;
    if !training.status(Local::now()).can_sign_out() {
        return Ok(DoneView::err(format!(
            "Запись на тренировку *{}* в {} *закрыта*",
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        ))
        .go_back()
        .into());
    }
    ctx.services
        .booking
        .sign_out(&mut ctx.session, training.id(), user_id, false)
        .await?;

    if !training.is_group() {
        let instructor = ctx
            .services
            .users
            .get_user(&mut ctx.session, training.instructor)
            .await?;
        ctx.bot
            .notify(
                ChatId(instructor.tg_id),
                &format!(
                    "Клиент {} {} отменил запись на персональную тренировку {}",
                    escape(&ctx.me.name.first_name),
                    fmt_phone(ctx.me.phone.as_deref()),
                    fmt_dt(&training.get_slot().start_at())
                ),
                true,
            )
            .await;
    }

    let msg = format!(
        "Вы отменили запись на тренировку *{}* в {}",
        escape(&training.name),
        fmt_dt(&training.get_slot().start_at())
    );
    Ok(DoneView::ok(msg).into())
}
