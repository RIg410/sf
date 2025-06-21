use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::{Jmp, View, ViewResult},
};
use decimal::Decimal;
use eyre::Result;
use ident::source::Source;
use rights::Rule;
use teloxide::{
    types::{InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub struct PayMarketing;

#[async_trait]
impl View for PayMarketing {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("Введите сумму оптаты за маркетинг", Default::default())
            .await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        let text = if let Some(msg) = message.text() {
            msg
        } else {
            return Ok(Jmp::Stay);
        };

        let amount = match text.parse::<u64>() {
            Ok(amount) => amount,
            Err(_) => {
                ctx.send_msg("Введите число").await?;
                return Ok(Jmp::Stay);
            }
        };

        Ok(Jmp::Next(
            ComeFromType {
                amount: Decimal::int(amount as i64),
            }
            .into(),
        ))
    }
}

struct ComeFromType {
    amount: Decimal,
}

#[async_trait]
impl View for ComeFromType {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = "Выберите категорию оплаты".to_string();

        let mut keymap = InlineKeyboardMarkup::default();

        for cf in Source::iter() {
            keymap = keymap.append_row(cf.btn_row(cf.name()));
        }

        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        let come_from: Source = calldata!(data);
        Ok(Jmp::Next(
            Confirm {
                amount: self.amount,
                come_from,
            }
            .into(),
        ))
    }
}

struct Confirm {
    amount: Decimal,
    come_from: Source,
}

#[async_trait]
impl ConfirmView for Confirm {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "Подтвердите оплату маркетинга на сумму {} *{}*",
            escape(&self.amount.to_string()),
            escape(self.come_from.name())
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::MakePayment)?;
        ctx.services
            .treasury
            .pay_for_marketing(&mut ctx.session, self.amount, self.come_from)
            .await?;

        Ok(DoneView::ok(format!(
            "Оплата за маркетинг _{}_ на сумму {} успешно добавлена",
            escape(self.come_from.name()),
            escape(&self.amount.to_string()),
        ))
        .into())
    }
}
