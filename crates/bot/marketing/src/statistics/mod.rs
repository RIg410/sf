use ai::AiModel;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::day::fmt_date;
use eyre::Error;
use eyre::Result;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;
use time::range::MonthRange;

mod advertising;

pub struct StatisticsView {
    range: MonthRange,
    ai: Option<AiModel>,
}

impl Default for StatisticsView {
    fn default() -> Self {
        Self {
            range: MonthRange::default(),
            ai: None,
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

        if ctx.has_right(Rule::AIStatistic) {
            if self.ai.is_none() {
                keymap = keymap.append_row(
                    Calldata::SetAi(Some(AiModel::Gpt4oMini)).btn_row("Использовать ИИ 🤖"),
                );
            } else {
                keymap = keymap.append_row(Calldata::SetAi(None).btn_row("Сбросить ИИ 🤖"));
            }
        }

        keymap = keymap.append_row(
            Calldata::Statistics(StatisticType::AdvertisingStat).btn_row("Конверсия каналов 📈"),
        );

        let (from, to) = self.range.range()?;
        ctx.edit_origin(
            &format!(
                "📊 Статистика с *{}* по *{}*",
                fmt_date(&from),
                fmt_date(&to)
            ),
            keymap,
        )
        .await?;

        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::ViewStatistics)?;

        match calldata!(data) {
            Calldata::NextMonth => {
                self.range = self.range.next()?;
            }
            Calldata::PrevMonth => {
                self.range = self.range.prev()?;
            }
            Calldata::SetAi(ai) => {
                self.ai = ai;
            }
            Calldata::Statistics(statistic_type) => match statistic_type {
                StatisticType::AdvertisingStat => {
                    statistic_type
                        .send_statistic(ctx, self.range, self.ai)
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
    SetAi(Option<AiModel>),
    Statistics(StatisticType),
}

#[derive(Serialize, Deserialize)]
pub enum StatisticType {
    AdvertisingStat,
}

impl StatisticType {
    pub async fn send_statistic(
        &self,
        ctx: &mut Context,
        range: MonthRange,
        ai: Option<AiModel>,
    ) -> Result<(), Error> {
        match self {
            StatisticType::AdvertisingStat => advertising::send_conversion(ctx, range, ai).await,
        }
    }
}
