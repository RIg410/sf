use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::Local;
use eyre::Result;
use rights::Rule;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};
use time::range::MonthRange;

pub struct Stat {
    range: Option<MonthRange>,
}

impl Stat {
    pub fn new(range: Option<MonthRange>) -> Self {
        Self { range }
    }
}

#[async_trait]
impl View for Stat {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.ensure(Rule::ViewFinance)?;

        let (from, to) = self
            .range
            .map(|r| r.range().map(|r| (Some(r.0), Some(r.1))))
            .transpose()?
            .unwrap_or((None, None));

        let stat = ctx
            .services
            .treasury
            .aggregate(&mut ctx.session, from, to)
            .await?;
        let mut text = format!(
            "📊Статистика с _{}_ по _{}_:\n",
            from.map(|f| f.format("%d\\.%m\\.%Y").to_string())
                .unwrap_or_else(|| "\\-".to_string()),
            to.map(|f| f.format("%d\\.%m\\.%Y").to_string())
                .unwrap_or_else(|| "\\-".to_string()),
        );

        writeln!(
            &mut text,
            "*Баланс*:_{}_",
            escape(&(stat.debit - stat.credit).to_string())
        )?;
        writeln!(&mut text, "*Поступления*:")?;
        writeln!(
            &mut text,
            "Проданно абонементов:_{}_ на сумму _{}_",
            stat.income.subscriptions.count,
            escape(&stat.income.subscriptions.sum.to_string())
        )?;
        writeln!(
            &mut text,
            "Субаренда: _{}_ на сумму _{}_",
            stat.income.sub_rent.count,
            escape(&stat.income.sub_rent.sum.to_string())
        )?;
        writeln!(
            &mut text,
            "Другие поступления:_{}_",
            escape(&stat.income.other.sum.to_string())
        )?;

        writeln!(&mut text, "*Расходы*:")?;
        writeln!(
            &mut text,
            "Выплачено вознаграждений: _{}_",
            escape(&stat.outcome.rewards.sum.to_string())
        )?;
        writeln!(
            &mut text,
            "Оплачено аренды: _{}_",
            escape(&stat.outcome.rent.sum.to_string())
        )?;
        writeln!(
            &mut text,
            "Другие расходы:_{}_",
            escape(&stat.outcome.other.sum.to_string())
        )?;

        writeln!(&mut text, "*Маркетинг*:")?;
        stat.outcome
            .marketing
            .iter()
            .try_for_each(|(come_from, sum)| {
                writeln!(
                    &mut text,
                    "_{}_: _{}_",
                    come_from.name(),
                    escape(&sum.sum.to_string())
                )
            })?;

        let mut keymap = InlineKeyboardMarkup::default();

        if let Some(range) = self.range {
            let date = range.base_date();
            let mut row = Vec::new();
            row.push(Calldata::PrevMonth.button("🔙"));

            if date < Local::now() {
                row.push(Calldata::NextMonth.button("🔜"));
            }

            keymap = keymap.append_row(row);
        }

        ctx.edit_origin(&text, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Calldata::NextMonth => {
                self.range = self.range.map(|r| r.next()).transpose()?;
            }
            Calldata::PrevMonth => {
                self.range = self.range.map(|r| r.prev()).transpose()?;
            }
        }
        Ok(Jmp::Stay)
    }
}

#[derive(Serialize, Deserialize)]
enum Calldata {
    NextMonth,
    PrevMonth,
}
