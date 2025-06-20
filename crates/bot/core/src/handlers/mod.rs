pub mod callback;
pub mod error;
pub mod message;

use std::sync::Arc;

use crate::{
    bot::{Origin, TgBot},
    context::Context,
    state::StateHolder,
    widget::{ViewResult, Widget},
};
use env::Env;
use error::handle_err;
use eyre::Error;
use services::SfServices;
use teloxide::{
    Bot,
    types::{ChatId, MessageId},
};
use users::model::User;

pub async fn build_context(
    bot: Bot,
    ledger: Arc<SfServices>,
    tg_id: ChatId,
    state_holder: &StateHolder,
    env: Env,
) -> Result<(Context, Option<Widget>), (Error, Bot)> {
    let mut session = ledger
        .db
        .start_anonymous_session()
        .await
        .map_err(|err| (err, bot.clone()))?;
    let (mut user, real) = if let Some(user) = ledger
        .users
        .get_by_tg_id(&mut session, tg_id.0)
        .await
        .map_err(|err| (err, bot.clone()))?
    {
        (user, true)
    } else {
        (User::with_tg_id(tg_id.0), false)
    };
    ledger
        .users
        .resolve_family(&mut session, &mut user)
        .await
        .map_err(|err| (err, bot.clone()))?;
    session.set_actor(user.id);
    let state = state_holder.get_state(tg_id).unwrap_or_default();

    let origin = if let Some(origin) = state.origin {
        origin
    } else {
        Origin {
            chat_id: tg_id,
            message_id: MessageId(0),
            tkn: state_holder.get_token(tg_id),
        }
    };

    let tg_bot = TgBot::new(bot, state_holder.tokens(), origin, env);

    Ok((
        Context::new(tg_bot, user, ledger, session, real),
        state.view,
    ))
}

pub(crate) fn handle_result(
    result: ViewResult,
    current: Widget,
    system_handler: impl Fn() -> Widget,
) -> Result<Widget, Error> {
    Ok(match handle_err(result)? {
        crate::widget::Jmp::Next(mut new_widget) => {
            new_widget.set_back(current);
            new_widget
        }
        crate::widget::Jmp::Stay => current,
        crate::widget::Jmp::Home => system_handler(),
        crate::widget::Jmp::Goto(widget) => widget,
        crate::widget::Jmp::Back(steps) => {
            let mut back = current;
            for _ in 0..steps {
                back = back.take_back().unwrap_or_else(&system_handler)
            }
            back
        }
        crate::widget::Jmp::ToSafePoint => {
            let mut back = current;
            loop {
                back = back.take_back().unwrap_or_else(&system_handler);
                if back.is_safe_point() {
                    break back;
                }
            }
        }
    })
}
