use super::jwt::{Claims, Jwt};
use bot_main::BotApp;
use bson::oid::ObjectId;
use std::sync::Arc;
use tonic::service::Interceptor;

#[derive(Clone)]
pub struct AuthInterceptor {
    jwt: Arc<Jwt>,
}

impl AuthInterceptor {
    pub fn new(bot: BotApp) -> Self {
        let jwt = Arc::new(Jwt::new(bot.env.jwt_secret()));
        AuthInterceptor { jwt }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let meta = request.metadata();
        let token = meta.get("Authorization").and_then(|v| v.to_str().ok());
        if let Some(token) = token {
            if let Ok((auth_key, _)) = self.jwt.claims::<Claims>(token) {
                request.extensions_mut().insert(UserId(auth_key.id));
            }
        }

        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct UserId(pub ObjectId);

impl From<UserId> for ObjectId {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}
