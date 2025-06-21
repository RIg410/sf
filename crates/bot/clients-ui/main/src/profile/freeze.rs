use async_trait::async_trait;
use bot_core::views::ask::AskView;
use bot_core::views::confirm::ConfirmView;
use bot_core::views::done::DoneView;
use bot_core::{
    context::Context,
    widget::{Jmp, ViewResult},
};
use eyre::Result;

pub struct UnfreezeConfirm;

#[async_trait]
impl ConfirmView for UnfreezeConfirm {
    async fn message(&self, _: &mut Context) -> Result<String> {
        Ok("Вы уверены, что хотите разморозить аккаунт?".to_string())
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        let me = &ctx.me;
        let client = me.as_client()?;
        if client.freeze.is_none() {
            return Ok(Jmp::ToSafePoint);
        }

        ctx.services
            .users
            .unfreeze(&mut ctx.session, ctx.me.id)
            .await?;
        Ok(DoneView::ok("Аккаунт разморожен").into())
    }
}

pub struct AskFreezeDays;

#[async_trait]
impl AskView<u32> for AskFreezeDays {
    const ERROR_MESSAGE: &'static str = "Введите число дней заморозки";

    async fn message(&self, ctx: &mut Context) -> eyre::Result<String> {
        let client = ctx.me.as_client()?;
        Ok(format!(
            "Осталось дней заморозки: _{}_\nНа сколько дней заморозить абонемент?",
            client.freeze_days
        ))
    }

    async fn on_answer(&self, ctx: &mut Context, value: u32) -> ViewResult {
        if value == 0 {
            ctx.send_msg("Введите число больше 0.").await?;
            return Ok(Jmp::Stay);
        }
        let client = ctx.me.as_client()?;
        if client.freeze_days < value {
            ctx.send_msg("У вас недостаточно дней заморозки.").await?;
            return Ok(Jmp::Stay);
        }

        Ok(FreezeConfirm::new(value).into())
    }
}

struct FreezeConfirm {
    days: u32,
}
impl FreezeConfirm {
    pub fn new(days: u32) -> Self {
        FreezeConfirm { days }
    }
}

#[async_trait]
impl ConfirmView for FreezeConfirm {
    async fn message(&self, _: &mut Context) -> Result<String> {
        Ok(format!(
            "Замораживаем Ваш абонемент\\. Количество дней: _{}_\nВсе верно?",
            self.days
        ))
    }

    async fn on_confirm(&self, ctx: &mut Context) -> ViewResult {
        ctx.services
            .users
            .freeze(&mut ctx.session, ctx.me.id, self.days, false)
            .await?;

        Ok(DoneView::ok("Абонемент заморожен").into())
    }
}
