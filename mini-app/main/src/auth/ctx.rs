use super::jwt::Jwt;
use bot_core::{
    bot::{Origin, TgBot, ValidToken},
    context::Context,
};
use bot_main::BotApp;
use bson::oid::ObjectId;
use env::Env;
use ledger::Ledger;
use std::sync::Arc;
use teloxide::types::{ChatId, MessageId};
use tonic::Status;

#[derive(Clone)]
pub struct ContextBuilder {
    pub ledger: Arc<Ledger>,
    bot: BotApp,
    jwt: Arc<Jwt>,
}

impl ContextBuilder {
    pub fn new(ledger: Arc<Ledger>, bot: BotApp) -> Self {
        let jwt = Arc::new(Jwt::new(bot.env.jwt_secret()));
        ContextBuilder { ledger, bot, jwt }
    }

    pub fn env(&self) -> &Env {
        &self.bot.env
    }

    pub async fn build(&self, id: ObjectId) -> Result<Context, Status> {
        let mut session = self
            .ledger
            .db
            .start_session()
            .await
            .map_err(|_| Status::internal("Failed to start session"))?;

        let user = self
            .ledger
            .users
            .get(&mut session, id)
            .await
            .map_err(|_| Status::internal(format!("Failed to get user by id: {}", id)))?;
        let mut user = if let Some(user) = user {
            user
        } else {
            return Err(Status::not_found(format!("User not found: {}", id)));
        };
        self.ledger
            .users
            .resolve_family(&mut session, &mut user)
            .await
            .map_err(|_| Status::internal(format!("Failed to resolve family for user: {}", id)))?;

        session.set_actor(user.id);

        let state = self
            .bot
            .state
            .get_state(ChatId(user.tg_id))
            .unwrap_or_default();

        let origin = if let Some(origin) = state.origin {
            origin
        } else {
            Origin {
                chat_id: ChatId(user.tg_id),
                message_id: MessageId(0),
                tkn: ValidToken::new(),
            }
        };

        let tg_bot = TgBot::new(
            self.bot.bot.clone(),
            self.bot.state.tokens(),
            origin,
            self.bot.env.clone(),
        );
        Ok(Context::new(
            tg_bot,
            user,
            self.ledger.clone(),
            session,
            true,
        ))
    }
}
