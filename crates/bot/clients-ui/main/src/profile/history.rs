use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_dt;
use chrono::Local;
use eyre::Result;
use history::model::{Action, ActionType, HistoryRow};
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

pub const LIMIT: u64 = 7;

#[derive(Default)]
pub struct HistoryList {
    offset: u64,
}

#[async_trait]
impl View for HistoryList {
    fn name(&self) -> &'static str {
        "HistoryList"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let mut logs = ctx
            .services
            .history
            .actor_logs(
                &mut ctx.session,
                ctx.me.id,
                Some(LIMIT as usize),
                self.offset as usize,
                vec![ActionType::SellSub, ActionType::FinalizedTraining],
            )
            .await?;
        let mut msg = "*История:*".to_string();

        let mut log_len = 0;
        while let Some(log) = logs.next(&mut ctx.session).await {
            log_len += 1;

            if let Ok(row_msg) = fmt_row(&log?) {
                msg.push_str(&format!("\n\n📌{}", row_msg));
            }
        }

        let mut keymap = vec![];
        if self.offset > 0 {
            keymap.push(Calldata::Offset(self.offset - LIMIT).button("⬅️"));
        }
        if log_len >= LIMIT {
            keymap.push(Calldata::Offset(self.offset + LIMIT).button("➡️"));
        }

        ctx.edit_origin(&msg, InlineKeyboardMarkup::new(vec![keymap]))
            .await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Calldata::Offset(offset) => {
                self.offset = offset;
                Ok(Jmp::Stay)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Calldata {
    Offset(u64),
}

fn fmt_row(log: &HistoryRow) -> Result<String> {
    let message = match &log.action {
        Action::SellSub {
            subscription,
            discount: _,
        } => {
            format!(
                "Вы купили абонемент *{}*\nКоличество занятий:_{}_\nСумма:_{}_",
                escape(&subscription.name),
                subscription.items,
                escape(&subscription.price.to_string())
            )
        }
        Action::FinalizedTraining {
            name,
            start_at,
            room_id: _,
        } => {
            format!(
                "Вы посетили тренировку *{}* в _{}_",
                escape(name),
                fmt_dt(&start_at.with_timezone(&Local))
            )
        }
        _ => {
            return Err(eyre::eyre!("Unknown action type: {:?}", log.action));
        }
    };

    Ok(format!(
        "{}:\n{}",
        fmt_dt(&log.date_time.with_timezone(&Local)),
        message
    ))
}
