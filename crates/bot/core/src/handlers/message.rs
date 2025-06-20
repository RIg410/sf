use super::handle_result;
use crate::{
    BACK_NAME, ERROR, HOME_NAME,
    context::Context,
    state::{State, StateHolder},
    widget::Widget,
};
use teloxide::{prelude::ResponseResult, types::Message, utils::markdown::escape};
use tracing::error;

pub async fn message_handler(
    msg: Message,
    ctx: &mut Context,
    widget: Widget,
    state_holder: &StateHolder,
    system_handler: impl Fn() -> Widget,
) -> ResponseResult<()> {
    match inner_message_handler(ctx, widget, msg, state_holder, system_handler).await {
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
    ctx.set_system_go_back(!widget.is_back_main_view() && !widget.main_view());
    ctx.set_system_go_home(true);

    if let Some(msg) = msg.text() {
        if msg.starts_with("/") {
            match msg {
                BACK_NAME => {
                    if let Some(back) = widget.take_back() {
                        widget = back;
                    } else {
                        widget = system_handler();
                    }
                }
                HOME_NAME => {
                    let mut system = system_handler();
                    system.show(ctx).await?;
                    widget = system;
                }
                _ => widget = system_handler(),
            }
        }
    }

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
