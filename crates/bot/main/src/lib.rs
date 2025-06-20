mod system;
mod view;
use std::sync::Arc;

use bot_client_main::ClientMain;
use bot_core::{
    context::Context,
    handlers::{build_context, callback::callback_handler, message::message_handler},
    state::{State, StateHolder},
    widget::{Jmp, View, Widget},
};
use env::Env;
use eyre::Result;
use services::SfServices;
use teloxide::{
    Bot,
    dispatching::UpdateFilterExt as _,
    dptree,
    prelude::{Dispatcher, Requester as _, ResponseResult},
    types::{CallbackQuery, ChatId, InlineQuery, Message, PreCheckoutQuery, Update},
};
use tracing::{error, info};
use view::menu::{MainMenuItem, MainMenuView};

use crate::view::signup::SignUpView;

const ERROR: &str = "Что-то пошло не так. Пожалуйста, попробуйте позже.";

#[derive(Clone)]
pub struct BotApp {
    pub bot: Bot,
    pub env: Env,
    pub state: StateHolder,
}

impl BotApp {
    pub fn new(env: Env) -> Self {
        BotApp {
            bot: Bot::new(env.tg_token()),
            state: StateHolder::default(),
            env,
        }
    }

    pub async fn start(self, ledger: Arc<SfServices>) -> Result<()> {
        let state = self.state;
        let bot = self.bot;
        bot.set_my_commands(vec![
            MainMenuItem::Home.into(),
            MainMenuItem::Profile.into(),
            MainMenuItem::Schedule.into(),
            MainMenuItem::Subscription.into(),
        ])
        .await?;

        let msg_sf = ledger.clone();
        let env_ledger = self.env.clone();
        let msg_state = state.clone();
        let env_state = self.env.clone();

        let callback_ledger = ledger.clone();
        let callback_state = state.clone();
        let handler = dptree::entry()
            .branch(
                Update::filter_message().endpoint(move |bot: Bot, msg: Message| {
                    on_message(
                        bot,
                        msg,
                        msg_sf.clone(),
                        env_ledger.clone(),
                        msg_state.clone(),
                    )
                }),
            )
            .branch(
                Update::filter_pre_checkout_query()
                    .endpoint(|bot: Bot, q: PreCheckoutQuery| pre_checkout_query_handler(bot, q)),
            )
            .branch(
                Update::filter_callback_query().endpoint(move |bot: Bot, q: CallbackQuery| {
                    on_callback(
                        bot,
                        q,
                        callback_ledger.clone(),
                        callback_state.clone(),
                        env_state.clone(),
                    )
                }),
            )
            .branch(
                Update::filter_inline_query().endpoint(move |bot: Bot, q: InlineQuery| {
                    inline_query_handler(bot, q, ledger.clone(), state.clone())
                }),
            );

        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

fn system_handler(ctx: &Context) -> fn() -> Widget {
    match &ctx.me.role {
        users::model::role::UserRole::Client(_) => || ClientMain::default().widget(),
        users::model::role::UserRole::Instructor(_)
        | users::model::role::UserRole::Manager(_)
        | users::model::role::UserRole::Admin(_) => || MainMenuView::default().widget(),
    }
}

async fn inline_query_handler(
    _: Bot,
    _: InlineQuery,
    _: Arc<SfServices>,
    _: StateHolder,
) -> ResponseResult<()> {
    info!("inline");
    Ok(())
}

async fn pre_checkout_query_handler(bot: Bot, q: PreCheckoutQuery) -> ResponseResult<()> {
    bot.answer_pre_checkout_query(q.id, true).await?;
    Ok(())
}

async fn on_message(
    bot: Bot,
    msg: Message,
    sf: Arc<SfServices>,
    env: Env,
    state_holder: StateHolder,
) -> ResponseResult<()> {
    dbg!(msg.text());
    let result = context(bot, sf, msg.chat.id, &state_holder, env).await;
    match result {
        ContextResult::Regular(mut ctx, widget) => {
            let system_handler = system_handler(&ctx);
            let widget = widget.unwrap_or(system_handler());

            message_handler(msg, &mut ctx, widget, &state_holder, system_handler).await
        }
        ContextResult::Anonymous(mut context, widget) => {
            on_auth(&mut context, widget, msg, &state_holder).await
        }
        ContextResult::FailedToBuild | ContextResult::Blocked => return Ok(()),
    }
}

async fn on_callback(
    bot: Bot,
    q: CallbackQuery,
    sf: Arc<SfServices>,
    state_holder: StateHolder,
    env: Env,
) -> ResponseResult<()> {
    let chat_id = if let Some(chat_id) = q.message.as_ref().map(|msg| msg.chat().id) {
        chat_id
    } else {
        return Ok(());
    };

    let result = context(bot, sf, chat_id, &state_holder, env).await;
    match result {
        ContextResult::Regular(mut ctx, widget) => {
            let system_handler = system_handler(&ctx);
            let widget = widget.unwrap_or(system_handler());

            callback_handler(q, &mut ctx, widget, state_holder, system_handler).await
        }
        ContextResult::Anonymous(mut context, _) => {
            let mut auth_view = SignUpView::default();
            if let Err(err) = auth_view.show(&mut context).await {
                error!("Failed to show SignUpView: {:#}", err);
                return Ok(());
            }
            Ok(())
        }
        ContextResult::FailedToBuild | ContextResult::Blocked => Ok(()),
    }
}

async fn context(
    bot: Bot,
    ledger: Arc<SfServices>,
    chat_id: ChatId,
    state_holder: &StateHolder,
    env: Env,
) -> ContextResult {
    match build_context(bot, ledger, chat_id, &state_holder, env).await {
        Ok((ctx, widget)) => {
            if ctx.is_real_user {
                if !ctx.is_active() {
                    if let Err(err) = ctx.send_msg("Ваш аккаунт заблокирован").await
                    {
                        error!("Failed to send message: {:#}", err);
                    }
                    return ContextResult::Blocked;
                } else {
                    ContextResult::Regular(ctx, widget)
                }
            } else {
                ContextResult::Anonymous(ctx, widget)
            }
        }
        Err((err, bot)) => {
            error!("Failed to build context: {}", err);
            if let Err(err) = bot.send_message(chat_id, ERROR).await {
                error!("Failed to send message: {:#}", err);
            }
            ContextResult::FailedToBuild
        }
    }
}

enum ContextResult {
    Regular(Context, Option<Widget>),
    Anonymous(Context, Option<Widget>),
    FailedToBuild,
    Blocked,
}

async fn on_auth(
    ctx: &mut Context,
    widget: Option<Widget>,
    msg: Message,
    state_holder: &StateHolder,
) -> ResponseResult<()> {
    let mut widget = if let Some(widget) = widget {
        widget
    } else {
        let mut auth_view = SignUpView::default();
        if let Err(err) = auth_view.show(ctx).await {
            error!("Failed to show SignUpView: {:#}", err);
            return Ok(());
        }
        state_holder.set_state(
            ctx.chat_id(),
            State {
                view: Some(auth_view.widget()),
                origin: Some(ctx.origin().clone()),
            },
        );
        return Ok(());
    };

    let result = widget.handle_message(ctx, &msg).await;

    match result {
        Ok(jmp) => match jmp {
            Jmp::Home => {
                let mut system_handler = system_handler(&ctx)();
                let _ = system_handler.show(ctx).await;

                state_holder.set_state(
                    ctx.chat_id(),
                    State {
                        view: Some(system_handler),
                        origin: Some(ctx.origin().clone()),
                    },
                );
                Ok(())
            }
            _ => Ok(()),
        },
        Err(err) => {
            error!("Failed to handle auth message: {:#}", err);
            Ok(())
        }
    }
}
