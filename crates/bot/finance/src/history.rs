use std::vec;

use async_trait::async_trait;
use bot_core::{
    context::Context,
    script::{
        Dispatch, ScriptView, Stage,
        list::{ListId, ListItem, StageList},
    },
    widget::{View, Widget},
};
use bot_viewer::day::fmt_dt;
use chrono::Local;
use eyre::{Error, Result, eyre};
use rights::Rule;
use teloxide::utils::markdown::escape;
use treasury::model::{Event, TreasuryEvent};

use crate::operation::FinanceOperation;

pub fn history_view() -> Widget {
    ScriptView::new(State {}, Stage::list(FinanceView {})).into()
}

pub struct State {}

pub struct FinanceView {}

#[async_trait]
impl StageList<State> for FinanceView {
    async fn message(
        &self,
        ctx: &mut Context,
        _: &mut State,
        limit: usize,
        offset: usize,
    ) -> Result<(String, Vec<Vec<ListItem>>)> {
        ctx.ensure(Rule::MakePayment)?;
        let page = ctx
            .services
            .treasury
            .page(&mut ctx.session, limit as u64, offset as u64)
            .await?;

        let mut items = vec![];
        let mut msg = "Финансовые операции                     💰💸".to_string();
        for (idx, event) in page.iter().enumerate() {
            let item = make_list_item(idx, event);
            msg.push_str(&format!(
                "\n\n{} _{}_\n{}",
                item.name,
                escape(&(event.debit - event.credit).to_string()),
                fmt_dt(&event.date_time.with_timezone(&Local))
            ));
            items.push(vec![item]);
        }
        Ok((msg, items))
    }

    fn back(&self) -> Option<Stage<State>> {
        None
    }

    async fn select(
        &self,
        ctx: &mut Context,
        _: &mut State,
        id: ListId,
    ) -> Result<Dispatch<State>, Error> {
        ctx.ensure(Rule::MakePayment)?;

        let id = id.as_object_id().ok_or_else(|| eyre!("Invalid id"))?;
        Ok(Dispatch::Widget(FinanceOperation::new(id).widget()))
    }
}

pub fn make_list_item(idx: usize, event: &TreasuryEvent) -> ListItem {
    let symbol = match &event.event {
        Event::SellSubscription(_) => format!("{idx} 📈 продажа абонемента"),
        Event::Reward(_) => format!("{idx} 📉 выплата зп"),
        Event::Outcome(out) => format!("{} 📉{}", idx, escape(&out.description)),
        Event::Income(income) => {
            format!("{} 📈{}", idx, escape(&income.description))
        }
        Event::SubRent => {
            format!("📈{idx} Суб аренда")
        }
        Event::Rent => {
            format!("📉{idx} Аренда")
        }
        Event::Marketing(come_from) => {
            format!("📊{} Маркетинг \\({}\\)", idx, come_from.name())
        }
    };

    ListItem {
        id: ListId::ObjectId(event.id.bytes()),
        name: symbol,
    }
}
