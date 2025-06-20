use crate::{
    BACK_NAME, ERROR,
    context::Context,
    handlers::handle_result,
    state::{State, StateHolder},
    widget::Widget,
};
use teloxide::{prelude::ResponseResult, types::CallbackQuery, utils::markdown::escape};
use tracing::error;

pub async fn callback_handler(
    q: CallbackQuery,
    ctx: &mut Context,
    widget: Widget,
    state_holder: StateHolder,
    system_handler: impl Fn() -> Widget,
) -> ResponseResult<()> {
    let result = match inner_callback_handler(
        ctx,
        widget,
        q.data.unwrap_or_default(),
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
    };
    ctx.bot.answer_callback_query(q.id).await?;
    result
}

async fn inner_callback_handler(
    ctx: &mut Context,
    mut widget: Widget,
    data: String,
    state_holder: &StateHolder,
    system_handler: impl Fn() -> Widget,
) -> Result<(), eyre::Error> {
    ctx.set_system_go_back(!widget.is_back_main_view() && !widget.main_view());
    ctx.set_system_go_home(true);

    if data.starts_with("/") {
        match data.as_str() {
            BACK_NAME => {
                if let Some(back) = widget.take_back() {
                    widget = back;
                } else {
                    widget = system_handler();
                }
            }
            _ => widget = system_handler(),
        }
    }

    let result = widget.handle_callback(ctx, data.as_str()).await;
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
