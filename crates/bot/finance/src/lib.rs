pub mod employees;
pub mod history;
pub mod in_out;
pub mod marketing;
pub mod operation;
pub mod rent;
pub mod stat;

use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use chrono::{Datelike as _, Local};
use employees::list::EmployeeList;
use eyre::Result;
use history::history_view;
use in_out::{Op, TreasuryOp};
use rights::Rule;
use serde::{Deserialize, Serialize};
use stat::Stat;
use teloxide::types::InlineKeyboardMarkup;
use time::range::MonthRange;

#[derive(Default)]
pub struct FinanceView;

#[async_trait]
impl View for FinanceView {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let text = "💰 Финансы:".to_string();
        let mut keymap = InlineKeyboardMarkup::default();

        if ctx.has_right(Rule::MakePayment) {
            keymap = keymap.append_row(Callback::Payment.btn_row("Оплатить 💳"));
            keymap = keymap.append_row(Callback::PayMarketing.btn_row("Оплата маркетинга 📈"));
            keymap = keymap.append_row(Callback::PayRent.btn_row("Оплата аренды 🏢"));
        }

        if ctx.has_right(Rule::MakeDeposit) {
            keymap = keymap.append_row(Callback::Deposit.btn_row("Внести средства 🤑"));
        }

        if ctx.has_right(Rule::ViewEmployees) {
            keymap = keymap.append_row(Callback::EmployeeList.btn_row("Сотрудники ❤️"));
        }

        keymap = keymap.append_row(Callback::StatAll.btn_row("Общая статистика 📊"));
        keymap = keymap.append_row(Callback::StatByMonth.btn_row("Статистика за месяц 📈"));

        keymap = keymap.append_row(Callback::History.btn_row("История 📜"));
        ctx.edit_origin(&text, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Payment => {
                ctx.ensure(Rule::MakePayment)?;
                Ok(TreasuryOp::new(Op::Payment).into())
            }
            Callback::Deposit => {
                ctx.ensure(Rule::MakeDeposit)?;
                Ok(TreasuryOp::new(Op::Deposit).into())
            }
            Callback::History => {
                ctx.ensure(Rule::ViewFinance)?;
                Ok(Jmp::Next(history_view()))
            }
            Callback::StatAll => {
                ctx.ensure(Rule::ViewFinance)?;
                Ok(Stat::new(None).into())
            }
            Callback::StatByMonth => {
                ctx.ensure(Rule::ViewFinance)?;
                Ok(Stat::new(Some(MonthRange::new(
                    Local::now().with_day(1).unwrap_or_default(),
                )))
                .into())
            }
            Callback::PayMarketing => {
                ctx.ensure(Rule::MakePayment)?;
                Ok(marketing::PayMarketing.into())
            }
            Callback::PayRent => {
                ctx.ensure(Rule::MakePayment)?;
                Ok(rent::PayRent.into())
            }
            Callback::EmployeeList => {
                ctx.ensure(Rule::ViewEmployees)?;
                Ok(EmployeeList::new().into())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    PayMarketing,
    Payment,
    PayRent,

    Deposit,

    History,
    StatByMonth,
    StatAll,

    EmployeeList,
}
