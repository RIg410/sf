use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_dt;
use chrono::Local;
use decimal::Decimal;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use recalc::AddRecalcReward;
use rewards::model::{Reward, RewardSource};
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

mod recalc;

pub const LIMIT: u64 = 7;

pub struct RewardsList {
    id: ObjectId,
    offset: u64,
}

impl RewardsList {
    pub fn new(id: ObjectId) -> Self {
        RewardsList { id, offset: 0 }
    }
}

#[async_trait]
impl View for RewardsList {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        if !ctx.is_me(self.id) && !ctx.has_right(Rule::ViewRewards) {
            return Err(eyre::eyre!("Недостаточно прав"));
        }

        let logs = ctx
            .services
            .rewards
            .get(&mut ctx.session, self.id, LIMIT as i64, self.offset)
            .await?;
        let mut msg = "*История начислений:*".to_string();
        for log in &logs {
            msg.push_str(&format!("\n\n📌{}", fmt_row(log, ctx).await?));
        }
        let mut keymap = vec![];
        if self.offset > 0 {
            keymap.push(Calldata::Offset(self.offset - LIMIT).button("⬅️"));
        }
        if logs.len() as u64 >= LIMIT {
            keymap.push(Calldata::Offset(self.offset + LIMIT).button("➡️"));
        }

        let mut keymap = InlineKeyboardMarkup::new(vec![keymap]);
        if ctx.has_right(Rule::RecalculateRewards) {
            keymap = keymap.append_row(Calldata::Recalculate.btn_row("Добавить перерасчет"));
        }

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Calldata::Offset(offset) => {
                self.offset = offset;
                Ok(Jmp::Stay)
            }
            Calldata::Recalculate => {
                ctx.ensure(Rule::RecalculateRewards)?;
                Ok(Jmp::Next(AddRecalcReward::new(self.id).into()))
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Calldata {
    Offset(u64),
    Recalculate,
}

async fn fmt_row(log: &Reward, ctx: &mut Context) -> Result<String> {
    Ok(match &log.source {
        RewardSource::Recalc { comment } => {
            format!(
                "*{}*\n начислено *{}* \\- _перерасчет_ \\- {}",
                fmt_dt(&log.created_at.with_timezone(&Local)),
                escape(&log.reward.to_string()),
                escape(comment)
            )
        }
        RewardSource::Fixed {} => {
            format!(
                "*{}*\n начислено *{}* \\- _фиксированное вознаграждение_",
                fmt_dt(&log.created_at.with_timezone(&Local)),
                escape(&log.reward.to_string())
            )
        }
        RewardSource::Training {
            training_id,
            name,
            user_originals,
            percent,
        } => {
            let mut from_users = String::new();
            for rew in user_originals {
                let user = ctx
                    .services
                    .users
                    .get_user(&mut ctx.session, rew.user)
                    .await?;
                from_users.push_str(&escape(&format!(
                    "- {} цена занятий :{}. {}% = {}\n",
                    user.name.first_name,
                    rew.lesson_price,
                    *percent * Decimal::int(100),
                    rew.lesson_price * *percent
                )));
            }

            format!(
                "*{}*\n начислено *{}* \\- тренировка '{}' \\- {}\nКлиенты:\n {}",
                fmt_dt(&log.created_at.with_timezone(&Local)),
                escape(&log.reward.to_string()),
                escape(name),
                fmt_dt(&training_id.start_at.with_timezone(&Local)),
                from_users
            )
        }
    })
}
