use ai::AiModel;
use async_trait::async_trait;
use bot_core::{
    callback_data::{CallbackDateTime, Calldata as _},
    calldata,
    context::Context,
    widget::{Jmp, View},
};
use bot_viewer::day::{fmt_date, fmt_dt};
use chrono::Local;
use eyre::Error;
use eyre::Result;
use model::{rights::Rule, statistics::range::Range};
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

mod advertising;
mod budget;
mod instructors;

pub struct StatisticsView {
    range: Range,
}

impl Default for StatisticsView {
    fn default() -> Self {
        Self {
            range: Range::Day(Local::now()),
        }
    }
}

impl StatisticsView {}

#[async_trait]
impl View for StatisticsView {
    fn name(&self) -> &'static str {
        "StatisticsView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<(), Error> {
        ctx.ensure(Rule::ViewStatistics)?;

        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row([
            Calldata::PrevMonth.button("⬅️"),
            Calldata::NextMonth.button("➡️"),
        ]);
        keymap = keymap
            .append_row(Calldata::Statistics(StatisticType::AdvertisingStat).btn_row("Реклама 📈"));

        let (from, to) = self.range.range()?;
        ctx.edit_origin(
            &format!(
                "📊 Статистика \nс *{}* по *{}*",
                fmt_date(&from),
                fmt_date(&to)
            ),
            keymap,
        )
        .await?;

        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> Result<Jmp> {
        ctx.ensure(Rule::ViewStatistics)?;

        match calldata!(data) {
            Calldata::NextMonth => {
                self.range = self.range.next()?;
            }
            Calldata::PrevMonth => {
                self.range = self.range.prev()?;
            }
            Calldata::Statistics(statistic_type) => match statistic_type {
                StatisticType::AdvertisingStat => {
                    statistic_type
                        .send_statistic(ctx, self.range.clone())
                        .await?;
                }
            },
        }
        Ok(Jmp::Stay)
    }
}

#[derive(Serialize, Deserialize)]
enum Calldata {
    NextMonth,
    PrevMonth,
    Statistics(StatisticType),
}

#[derive(Serialize, Deserialize)]
pub enum StatisticType {
    AdvertisingStat,
}

impl StatisticType {
    pub async fn send_statistic(&self, ctx: &mut Context, range: Range) -> Result<(), Error> {
        match self {
            StatisticType::AdvertisingStat => advertising::send_statistic(ctx, range).await,
        }
    }
}
