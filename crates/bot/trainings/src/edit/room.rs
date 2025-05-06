use crate::schedule::group::set_date_time::TimeParts;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::{Local, Timelike, Utc};
use eyre::Result;
use ident::{rooms::Room, slot::Slot, training::TrainingId};
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardMarkup, Message};
use tracing::warn;

pub struct ChangeRoom {
    id: TrainingId,
    all: bool,
}

impl ChangeRoom {
    pub fn new(id: TrainingId, all: bool) -> ChangeRoom {
        ChangeRoom { id, all }
    }
}

#[async_trait]
impl View for ChangeRoom {
    fn name(&self) -> &'static str {
        "ChangeTime"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::ChangeTrainingSlot)?;

        let msg = "Выберете помещение";
        let keymap = InlineKeyboardMarkup::default();
        let mut keymap = keymap.append_row(vec![
            Room::Adult.button("👨‍🏫 Зал для взрослых"),
            Room::Child.button("👶 Зал для детей"),
        ]);

        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::ChangeTrainingSlot)?;
        let room: Room = calldata!(data);

        let training = ctx
            .services
            .calendar
            .get_training_by_id(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Training not found"))?;

        Ok(Jmp::Stay)

    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
     //  ctx.delete_msg(msg.id).await?;
        // let msg = if let Some(msg) = msg.text() {
        //     msg
        // } else {
        //     return Ok(Jmp::Stay);
        // };

        // let parts = match TimeParts::try_from(msg) {
        //     Ok(parts) => parts,
        //     Err(err) => {
        //         warn!("Invalid time format: {}", err);
        //         ctx.send_msg("Неверный формат времени\\.").await?;
        //         return Ok(Jmp::Stay);
        //     }
        // };
        // let hours = parts.0;
        // let minute = parts.1;

        // let start_at = if let Some(start_at) = self
        //     .id
        //     .start_at
        //     .with_timezone(&Local)
        //     .with_hour(hours)
        //     .and_then(|t| t.with_minute(minute))
        // {
        //     start_at.with_timezone(&Utc)
        // } else {
        //     ctx.send_msg("Неверный формат времени\\.").await?;
        //     return Ok(Jmp::Stay);
        // };

        // let training = ctx
        //     .services
        //     .calendar
        //     .get_training_by_id(&mut ctx.session, self.id)
        //     .await?
        //     .ok_or_else(|| eyre::eyre!("Training not found"))?;

        // Ok(ConfirmChangeTime::new(
        //     self.id,
        //     Slot::new(start_at, training.duration_min, self.id.room),
        //     self.all,
        // )
        // .into())
        Ok(Jmp::Stay)
    }
}

// pub struct ConfirmChangeTime {
//     id: TrainingId,
//     slot: Slot,
//     all: bool,
// }

// impl ConfirmChangeTime {
//     pub fn new(id: TrainingId, slot: Slot, all: bool) -> ConfirmChangeTime {
//         ConfirmChangeTime { id, slot, all }
//     }
// }

// #[async_trait]
// impl View for ConfirmChangeTime {
//     fn name(&self) -> &'static str {
//         "ConfirmChangeTime"
//     }

//     async fn show(&mut self, ctx: &mut Context) -> Result<()> {
//         ctx.ensure(Rule::ChangeTrainingSlot)?;

//         let msg = if self.all {
//             format!(
//                 "Изменить место тренировок с {} на {}?",
//                 Room::from(self.id.room),
//                 self.slot.start_at().format("%H:%M")
//             )
//         } else {
//             format!(
//                 "Изменить место тренировки с {} на {}?",
//                 self.id.start_at.with_timezone(&Local).format("%H:%M"),
//                 self.slot.start_at().format("%H:%M")
//             )
//         };

//         let mut keymap = InlineKeyboardMarkup::default();
//         keymap = keymap.append_row(vec![
//             ConfirmCallback::Confirm.button("✅ Подтвердить"),
//             ConfirmCallback::Cancel.button("❌ Отмена"),
//         ]);
//         ctx.edit_origin(&msg, keymap).await?;
//         Ok(())
//     }

//     async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
//         match calldata!(data) {
//             ConfirmCallback::Confirm => {
//                 ctx.ensure(Rule::ChangeTrainingSlot)?;

//                 ctx.services
//                     .calendar
//                     .change_slot(&mut ctx.session, self.id, self.slot, self.all)
//                     .await?;
//                 ctx.send_notification("Время тренировки изменено").await;
//                 Ok(Jmp::BackSteps(4))
//             }
//             ConfirmCallback::Cancel => Ok(Jmp::BackSteps(2)),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// enum ConfirmCallback {
//     Confirm,
//     Cancel,
// }
