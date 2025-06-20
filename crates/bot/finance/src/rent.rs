use async_trait::async_trait;
use bot_core::{
    context::Context,
    views::{confirm::ConfirmView, done::DoneView},
    widget::{Jmp, View, ViewResult},
};
use decimal::Decimal;
use eyre::Result;
use rights::Rule;
use teloxide::{types::Message, utils::markdown::escape};

pub struct PayRent;

#[async_trait]
impl View for PayRent {

    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        ctx.edit_origin("Введите сумму оптаты за аренду", Default::default())
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
            Confirm {
                amount: Decimal::int(amount as i64),
            }
            .into(),
        ))
    }
}
struct Confirm {
    amount: Decimal,
}

#[async_trait]
impl ConfirmView for Confirm {
    async fn message(&self, _: &mut Context) -> Result<String> {
        let msg = format!(
            "Подтвердите оплату аренду на сумму {}",
            escape(&self.amount.to_string()),
        );
        Ok(msg)
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.ensure(Rule::MakePayment)?;
        ctx.services
            .treasury
            .payment_rent(&mut ctx.session, self.amount)
            .await?;

        Ok(DoneView::ok(format!(
            "Оплата за аренду на сумму {} успешно добавлена",
            escape(&self.amount.to_string()),
        ))
        .into())
    }
}
