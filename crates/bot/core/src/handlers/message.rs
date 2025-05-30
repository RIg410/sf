use std::sync::Arc;

use super::{build_context, handle_result};
use crate::{
    BACK_NAME, ERROR,
    context::Context,
    state::{State, StateHolder},
    widget::Widget,
};
use env::Env;
use services::SfServices;
use teloxide::{
    Bot,
    prelude::{Requester as _, ResponseResult},
    types::Message,
    utils::markdown::escape,
};
use tracing::error;

pub async fn message_handler(
    bot: Bot,
    env: Env,
    msg: Message,
    ledger: Arc<SfServices>,
    state_holder: StateHolder,
    system_handler: impl Fn() -> Widget,
) -> ResponseResult<()> {
    let (mut ctx, widget) = match build_context(bot, ledger, msg.chat.id, &state_holder, env).await
    {
        Ok(ctx) => ctx,
        Err((err, bot)) => {
            error!("Failed to build context: {:#}", err);
            bot.send_message(msg.chat.id, ERROR).await?;
            return Ok(());
        }
    };

    match inner_message_handler(
        &mut ctx,
        widget.unwrap_or_else(&system_handler),
        msg,
        &state_holder,
        system_handler,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Failed to handle message: {:#}", err);
            if ctx.is_admin() {
                if let Err(err) = ctx
                    .send_msg(&escape(&format!("Failed to handle message: {err:#}")))
                    .await
                {
                    error!("send message error :{:#}", err);
                }
            } else if let Err(err) = ctx.send_msg(&escape(ERROR)).await {
                error!("send message error :{:#}", err);
            }
            Ok(())
        }
    }
}

async fn inner_message_handler(
    ctx: &mut Context,
    mut widget: Widget,
    msg: Message,
    state_holder: &StateHolder,
    system_handler: impl Fn() -> Widget,
) -> Result<(), eyre::Error> {
    if !ctx.is_active() {
        ctx.send_msg("Ваш аккаунт заблокирован").await?;
        return Ok(());
    }

    ctx.set_system_go_back(!widget.is_back_main_view() && !widget.main_view());
    ctx.set_system_go_home(true);

    let widget = if let Some(msg) = msg.text() {
        if msg.starts_with("/") {
            match msg {
                BACK_NAME => {
                    if let Some(mut back) = widget.take_back() {
                        back.show(ctx).await?;
                        back
                    } else {
                        system_handler()
                    }
                }
                "/start" => {
                    let mut widget = system_handler();
                    ctx.origin().invalidate();
                    widget.show(ctx).await?;
                    widget
                }
                _ => system_handler(),
            }
        } else {
            widget
        }
    } else {
        widget
    };

    let mut widget = if !ctx.is_real_user && !widget.allow_unsigned_user() {
        let mut handler = system_handler();
        handler.show(ctx).await?;
        handler
    } else {
        widget
    };

    let result = widget.handle_message(ctx, &msg).await;
    let mut new_widget = handle_result(result, widget, system_handler)?;
    ctx.set_system_go_back(!new_widget.is_back_main_view());
    ctx.set_system_go_home(true);

    new_widget.show(ctx).await?;

    state_holder.set_state(
        ctx.chat_id(),
        State {
            view: Some(new_widget),
            origin: Some(ctx.origin().clone()),
        },
    );
    Ok(())
}
