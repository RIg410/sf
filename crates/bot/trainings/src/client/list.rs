use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_dt;
use chrono::Local;
use eyre::{Result, bail};
use ident::training::TrainingId;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

use crate::client::{ClientView, Reason};

use super::add::AddClientView;

pub struct ClientsList {
    id: TrainingId,
}

impl ClientsList {
    pub fn new(id: TrainingId) -> Self {
        Self { id }
    }

    pub async fn view_user_profile(&mut self, id: ObjectId) -> ViewResult {
        Ok(ClientView::new(id, self.id, Reason::RemoveClient).into())
    }

    pub async fn add_client(&mut self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::EditTrainingClientsList)?;
        Ok(AddClientView::new(self.id).into())
    }

    pub async fn delete_client(&mut self, ctx: &mut Context, id: ObjectId) -> ViewResult {
        ctx.ensure(Rule::EditTrainingClientsList)?;

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        if training.is_processed {
            ctx.send_notification("Тренировка завершена\\. *Редактирование запрещено\\.*")
                .await;
            return Ok(Jmp::Stay);
        }
        ctx.services
            .booking
            .sign_out(&mut ctx.session, training.id(), id, true)
            .await?;
        ctx.send_notification("Клиент удален из тренировки").await;
        Ok(Jmp::Stay)
    }
}

#[async_trait]
impl View for ClientsList {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        if !ctx.is_employee() && !ctx.has_right(Rule::EditTrainingClientsList) {
            bail!("Only couch can see client list");
        }

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        let mut msg = format!(
            "📅 *{}*\n{}\n*Список участников:*\n",
            fmt_dt(&self.id.start_at.with_timezone(&Local)),
            escape(&training.name)
        );
        if training.is_processed {
            msg.push_str("Тренировка завершена\\. *Редактирование запрещено\\.*");
        }

        let mut keymap = InlineKeyboardMarkup::default();
        for client in &training.clients {
            let user = ctx
                .services
                .users
                .get(&mut ctx.session, *client)
                .await?
                .ok_or_else(|| eyre::eyre!("User not found"))?;
            let user_name = format!(
                "{} {}",
                user.name.first_name,
                user.name.tg_user_name.unwrap_or_default()
            );
            let mut row = Vec::with_capacity(2);
            row.push(Callback::SelectClient(user.id.bytes()).button(format!("👤 {user_name}")));
            if ctx.has_right(Rule::EditTrainingClientsList)
                && !training.is_processed
                && training.is_group()
            {
                row.push(Callback::DeleteClient(user.id.bytes()).button("❌"));
            }
            keymap = keymap.append_row(row);
        }

        if training.is_group()
            && ctx.has_right(Rule::EditTrainingClientsList)
            && !training.is_processed
        {
            keymap = keymap.append_row(vec![Callback::AddClient.button("Добавить 👤")]);
        }
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::SelectClient(id) => self.view_user_profile(ObjectId::from_bytes(id)).await,
            Callback::AddClient => self.add_client(ctx).await,
            Callback::DeleteClient(id) => self.delete_client(ctx, ObjectId::from_bytes(id)).await,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    SelectClient([u8; 12]),
    AddClient,
    DeleteClient([u8; 12]),
}
