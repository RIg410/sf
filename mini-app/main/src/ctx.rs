use crate::auth::jwt::{Claims, Jwt};
use bot_core::{
    bot::{Origin, TgBot, ValidToken},
    context::Context,
};
use bot_main::BotApp;
use bson::oid::ObjectId;
use env::Env;
use services::Services;
use tracing::warn;
use std::sync::Arc;
use teloxide::types::{ChatId, MessageId};
use tokio::time::sleep;
use tonic::Status;

#[derive(Clone)]
pub struct ContextBuilder {
    pub ledger: Arc<Services>,
    bot: BotApp,
    jwt: Arc<Jwt>,
}

impl ContextBuilder {
    pub fn new(ledger: Arc<Services>, bot: BotApp) -> Self {
        let jwt = Arc::new(Jwt::new(bot.env.jwt_secret()));
        ContextBuilder { ledger, bot, jwt }
    }

    pub async fn with_request<T>(&self, request: &tonic::Request<T>) -> Result<Context, Status> {
        let meta = request.metadata();
        let token = meta.get("Authorization").and_then(|v| v.to_str().ok());
        if let Some(token) = token {
            if let Ok((claim, _)) = self.jwt.claims::<Claims>(token) {
                self.build(claim.id).await
            } else {
                sleep(std::time::Duration::from_secs(1)).await;
                warn!("Failed to decode token: {}", token);
                Err(Status::unauthenticated("Invalid token"))
            }
        } else {
            sleep(std::time::Duration::from_secs(1)).await;
            Err(Status::unauthenticated("No Authorization header"))
        }
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

#[derive(Debug, Clone)]
pub struct UserId(pub ObjectId);

impl From<UserId> for ObjectId {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}
