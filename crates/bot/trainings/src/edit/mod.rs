use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_dt;
use couch::ChangeCouch;
use delete::ConfirmDeleteTraining;
use eyre::Result;
use ident::training::TrainingId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

pub mod couch;
pub mod delete;
pub mod name;
pub mod program;
pub mod slot;

pub struct EditTraining {
    id: TrainingId,
}

impl EditTraining {
    pub fn new(id: TrainingId) -> Self {
        Self { id }
    }

    pub fn hidden(ctx: &mut Context) -> Result<bool> {
        let show = ctx.has_right(Rule::EditTraining)
            || ctx.has_right(Rule::EditTrainingCouch)
            || ctx.has_right(Rule::RemoveTraining)
            || ctx.has_right(Rule::SetKeepOpen)
            || ctx.has_right(Rule::SetFree);
        Ok(!show)
    }

    async fn change_couch(&mut self, ctx: &mut Context, all: bool) -> ViewResult {
        ctx.ensure(Rule::EditTrainingCouch)?;
        Ok(ChangeCouch::new(self.id, all).into())
    }

    async fn delete_training(&mut self, ctx: &mut Context, all: bool) -> ViewResult {
        ctx.ensure(Rule::RemoveTraining)?;
        Ok(ConfirmDeleteTraining::new(self.id, all).into())
    }

    async fn keep_open(&mut self, ctx: &mut Context, keep_open: bool) -> ViewResult {
        ctx.ensure(Rule::SetKeepOpen)?;
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        if !training.is_group() {
            Err(eyre::eyre!("Can't delete personal training"))?;
        }
        ctx.services
            .calendar
            .set_keep_open(&mut ctx.session, training.id(), keep_open)
            .await?;
        Ok(Jmp::Stay)
    }

    async fn set_free(&mut self, ctx: &mut Context, free: bool) -> ViewResult {
        ctx.ensure(Rule::SetFree)?;

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;
        if !training.is_group() {
            Err(eyre::eyre!("Can't delete personal training"))?;
        }
        if !training.clients.is_empty() {
            ctx.send_msg("Нельзя изменить статус тренировки, на которую записаны клиенты")
                .await?;
            return Ok(Jmp::Stay);
        }

        let mut tp = training.tp;
        tp.set_is_free(free);

        ctx.services
            .calendar
            .set_training_type(&mut ctx.session, training.id(), tp)
            .await?;
        Ok(Jmp::Stay)
    }
}

#[async_trait]
impl View for EditTraining {

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        let msg = format!(
            "Редактировать тренировку *{}*\nв _{}_",
            escape(&training.name),
            fmt_dt(&training.get_slot().start_at())
        );

        let mut keymap = InlineKeyboardMarkup::default();

        if ctx.has_right(Rule::EditTraining) {
            keymap = keymap.append_row(vec![Callback::ChangeName.button("🔄 Изменить название")]);
            let mut row = vec![Callback::ChangeProgram(false).button("🔄 Изменить программу")];

            if !training.is_one_time {
                row.push(Callback::ChangeProgram(true).button("для всех"));
            }
            keymap = keymap.append_row(row);
        }

        if ctx.has_right(Rule::ChangeTrainingSlot) {
            let mut row = vec![Callback::ChangeStartAt(false).button("🕒 Изменить время")];

            if !training.is_one_time {
                row.push(Callback::ChangeStartAt(true).button("для всех"));
            }
            keymap = keymap.append_row(row);
        }

        if ctx.has_right(Rule::ChangeTrainingSlot) {
            let mut row = vec![Callback::ChangeRoom(false).button("🏢 Изменить зал")];

            if !training.is_one_time {
                row.push(Callback::ChangeRoom(true).button("для всех"));
            }
            keymap = keymap.append_row(row);
        }

        if ctx.has_right(Rule::RemoveTraining) {
            let mut row = vec![Callback::Delete(false).button("🗑️ Удалить эту тренировку")];

            if !training.is_one_time {
                row.push(Callback::Delete(true).button("все"));
            }
            keymap = keymap.append_row(row);
        }

        if ctx.has_right(Rule::EditTrainingCouch) {
            let mut row = vec![Callback::ChangeCouch(false).button("🔄 Изменить инструктора")];
            if !training.is_one_time {
                row.push(Callback::ChangeCouch(true).button("для всех"));
            }
            keymap = keymap.append_row(row);
        }

        if ctx.has_right(Rule::SetKeepOpen) {
            if training.keep_open {
                keymap = keymap.append_row(vec![
                    Callback::KeepOpen(false).button("🔒 Закрыть для записи"),
                ]);
            } else {
                keymap = keymap.append_row(vec![
                    Callback::KeepOpen(true).button("🔓 Открыть для записи"),
                ]);
            }
        }

        if ctx.has_right(Rule::SetFree) {
            if training.tp.is_free() {
                keymap =
                    keymap.append_row(vec![Callback::SetFree(false).button("Сделать платной")]);
            } else {
                keymap =
                    keymap.append_row(vec![Callback::SetFree(true).button("Сделать бесплатной")]);
            }
        }

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::ChangeCouch(all) => self.change_couch(ctx, all).await,
            Callback::Delete(all) => self.delete_training(ctx, all).await,
            Callback::KeepOpen(keep_open) => self.keep_open(ctx, keep_open).await,
            Callback::SetFree(free) => self.set_free(ctx, free).await,
            Callback::ChangeName => {
                if ctx.has_right(Rule::EditTraining) {
                    Ok(name::ChangeName::new(self.id).into())
                } else {
                    Ok(Jmp::Stay)
                }
            }
            Callback::ChangeStartAt(all) => {
                if ctx.has_right(Rule::ChangeTrainingSlot) {
                    Ok(slot::ChangeTime::new(self.id, all).into())
                } else {
                    Ok(Jmp::Stay)
                }
            }
            Callback::ChangeRoom(all) => {
                if ctx.has_right(Rule::ChangeTrainingSlot) {
                    Ok(slot::ChangeRoom::new(self.id, all).into())
                } else {
                    Ok(Jmp::Stay)
                }
            }
            Callback::ChangeProgram(all) => {
                if ctx.has_right(Rule::EditTraining) {
                    Ok(program::ChangeProgram::new(self.id, all).into())
                } else {
                    Ok(Jmp::Stay)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    ChangeCouch(bool),
    Delete(bool),
    KeepOpen(bool),
    SetFree(bool),
    ChangeStartAt(bool),
    ChangeName,
    ChangeProgram(bool),
    ChangeRoom(bool),
}
