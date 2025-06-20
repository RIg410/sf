use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{View, ViewResult},
};
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

pub mod requests;
mod statistics;

#[derive(Default)]
pub struct Marketing {}

#[async_trait]
impl View for Marketing {

    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.ensure(Rule::ViewMarketingInfo)?;
        let text = "Маркетинг🚀";
        let mut keymap = InlineKeyboardMarkup::default();

        if ctx.has_right(Rule::ViewMarketingInfo) {
            keymap = keymap.append_row(Calldata::Request.btn_row("Заявки 🈸"));
        }
        if ctx.has_right(Rule::ViewStatistics) {
            keymap = keymap.append_row(Calldata::Statistics.btn_row("Статистика 📊"));
        }

        ctx.bot.edit_origin(text, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Calldata::Request => {
                ctx.ensure(Rule::ViewMarketingInfo)?;
                Ok(requests::Requests::default().into())
            }
            Calldata::Statistics => {
                ctx.ensure(Rule::ViewStatistics)?;
                Ok(statistics::StatisticsView::default().into())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Calldata {
    Request,
    Statistics,
}
