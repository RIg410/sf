use async_trait::async_trait;
use booking::payer::AvailableBalance as _;
use bot_core::{
    CommonLocation,
    callback_data::Calldata as _,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::{
    fmt_phone,
    user::{link_to_user, render_profile_msg},
};
use eyre::Result;
use ident::training::TrainingId;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{ChatId, InlineKeyboardMarkup},
    utils::markdown::escape,
};
use trainings::model::Training;

pub mod add;
pub mod list;

pub struct ClientView {
    id: ObjectId,
    training_id: TrainingId,
    reason: Reason,
}

impl ClientView {
    pub fn new(id: ObjectId, training_id: TrainingId, reason: Reason) -> ClientView {
        ClientView {
            id,
            reason,
            training_id,
        }
    }

    async fn training(&self, ctx: &mut Context) -> Result<Training> {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.training_id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        Ok(training)
    }

    async fn add_client(&self, ctx: &mut Context) -> Result<()> {
        let training = self.training(ctx).await?;

        if training.is_processed {
            ctx.send_notification("Тренировка завершена\\. *Редактирование запрещено\\.*")
                .await;
            return Ok(());
        }
        ctx.services
            .booking
            .sign_up(&mut ctx.session, training.id(), self.id, true)
            .await?;

        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.id)
            .await?;

        ctx.send_notification(&format!(
            "{} добавлен в тренировку",
            escape(&user.name.first_name)
        ))
        .await;

        let payer = user.payer()?;
        let balance = payer.available_balance_for_training(&training);
        if balance <= 1 {
            if let Ok(users) = ctx
                .services
                .users
                .find_users_with_right(
                    &mut ctx.session,
                    Rule::ReceiveNotificationsAboutSubscriptions,
                )
                .await
            {
                for user in users {
                    ctx.bot
                        .notify_with_markup(
                            ChatId(user.tg_id),
                            &format!(
                                "У {} {} заканчивается абонемент\\.",
                                link_to_user(payer.as_ref()),
                                fmt_phone(payer.as_ref().phone.as_deref())
                            ),
                            InlineKeyboardMarkup::default().append_row(vec![
                                CommonLocation::Profile(payer.as_ref().id).button(),
                            ]),
                        )
                        .await;
                }
            }
        }

        Ok(())
    }

    async fn remove_client(&self, ctx: &mut Context) -> Result<()> {
        let training = self.training(ctx).await?;

        if training.is_processed {
            ctx.send_notification("Тренировка завершена\\. *Редактирование запрещено\\.*")
                .await;
            return Ok(());
        }
        ctx.services
            .booking
            .sign_out(&mut ctx.session, training.id(), self.id, true)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl View for ClientView {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let (msg, _, _) = render_profile_msg(ctx, self.id).await?;
        let mut keymap = InlineKeyboardMarkup::default();
        let training = self.training(ctx).await?;
        if training.is_group() {
            match self.reason {
                Reason::AddClient => {
                    keymap =
                        keymap.append_row(vec![Callback::AddClient.button("Добавить клиента 👤")]);
                }
                Reason::RemoveClient => {
                    keymap = keymap
                        .append_row(vec![Callback::DeleteClient.button("Удалить клиента ❌")]);
                }
            }
        }
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb = if let Some(cb) = Callback::from_data(data) {
            cb
        } else {
            return Ok(Jmp::Stay);
        };

        ctx.ensure(Rule::EditTrainingClientsList)?;

        match cb {
            Callback::GoBack => {}
            Callback::AddClient => {
                if let Reason::AddClient = self.reason {
                    self.add_client(ctx).await?;
                    return Ok(Jmp::Back(2));
                } else {
                    return Ok(Jmp::Stay);
                }
            }
            Callback::DeleteClient => {
                if let Reason::RemoveClient = self.reason {
                    self.remove_client(ctx).await?;
                } else {
                    return Ok(Jmp::Stay);
                }
            }
        };
        Ok(Jmp::Back(1))
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    GoBack,
    DeleteClient,
    AddClient,
}

#[derive(Clone, Copy)]
pub enum Reason {
    AddClient,
    RemoveClient,
}
