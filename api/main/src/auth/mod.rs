use crate::ctx::ContextBuilder;
use crate::pb::auth::auth_service_server::AuthService;
use crate::pb::auth::{
    SendVerificationCodeError, SendVerificationCodeResponse, TgAuthError, TgAuthResult,
    TgKeyRequest, VerificationCodeRequest, VerifyCodeError, VerifyCodeRequest, VerifyCodeResponse,
};
use codes::{AuthResult, Codes, PhoneNumber};
use jwt::{Claims, Jwt};
use tracing::debug;
use tracing::warn;
use std::sync::Arc;
use tg_token::TgAuth;
use tokio::time::sleep;
use tonic::async_trait;

pub mod codes;
pub mod jwt;
pub mod tg_token;

#[derive(Clone)]
pub struct AuthServer {
    context_builder: ContextBuilder,
    tg_auth: TgAuth,
    jwt: Arc<Jwt>,
    codes: Codes,
}

impl AuthServer {
    pub fn new(context_builder: ContextBuilder) -> Self {
        let tg_auth = TgAuth::new(context_builder.env().tg_token());
        let jwt = Arc::new(Jwt::new(context_builder.env().jwt_secret()));
        AuthServer {
            tg_auth,
            jwt,
            codes: Codes::default(),
            context_builder,
        }
    }
}

#[async_trait]
impl AuthService for AuthServer {
    async fn tg_auth(
        &self,
        request: tonic::Request<TgKeyRequest>,
    ) -> Result<tonic::Response<TgAuthResult>, tonic::Status> {
        let key = request.into_inner();
        let tg_id = match self.tg_auth.validate(&key.key) {
            Ok(id) => id,
            Err(err) => {
                sleep(std::time::Duration::from_secs(1)).await;

                warn!("Failed to validate key: {}", err);
                return Ok(tonic::Response::new(TgAuthResult {
                    token: None,
                    error: Some(TgAuthError::InvalidToken as i32),
                }));
            }
        };

        let mut session = self
            .context_builder
            .ledger
            .db
            .start_session()
            .await
            .map_err(|_| tonic::Status::internal("Failed to start session"))?;
        let user = self
            .context_builder
            .ledger
            .users
            .get_by_tg_id(&mut session, tg_id)
            .await
            .map_err(|_| tonic::Status::internal("Failed to get user by tg_id"))?;

        if let Some(user) = user {
            let jwt = self
                .jwt
                .make_jwt(Claims::new(user.id))
                .map_err(|_| tonic::Status::internal("Failed to create JWT"))?;
            return Ok(tonic::Response::new(TgAuthResult {
                token: Some(jwt.key),
                error: None,
            }));
        } else {
            sleep(std::time::Duration::from_secs(1)).await;

            return Ok(tonic::Response::new(TgAuthResult {
                token: None,
                error: Some(TgAuthError::UserNotFound as i32),
            }));
        }
    }

    async fn send_verification_code(
        &self,
        request: tonic::Request<VerificationCodeRequest>,
    ) -> Result<tonic::Response<SendVerificationCodeResponse>, tonic::Status> {
        debug!("send_verification_code: {:?}", request);
        let request = request.into_inner();
        let phone_number = PhoneNumber::from(request.phone_number.as_str());
        let code = self.codes.get_code(&phone_number);
        if let Some(code) = code {
            if code.is_valid() {
                debug!("Code already sent: {:?}", code);
                return Ok(tonic::Response::new(SendVerificationCodeResponse {
                    left_time: Some(code.left_time() as i32),
                    error: Some(SendVerificationCodeError::AlreadySent as i32),
                }));
            }
        }

        let mut session = self
            .context_builder
            .ledger
            .db
            .start_session()
            .await
            .map_err(|_| tonic::Status::internal("Failed to start session"))?;
        let user = self
            .context_builder
            .ledger
            .users
            .get_by_phone(&mut session, phone_number.as_ref())
            .await
            .map_err(|_| tonic::Status::internal("Failed to get user by phone"))?;
        if let Some(user) = user {
            let tg_id = user.tg_id;
            if tg_id == -1 {
                debug!("User not found: {}", user.id);
                return Ok(tonic::Response::new(SendVerificationCodeResponse {
                    left_time: None,
                    error: Some(SendVerificationCodeError::NotAvailable as i32),
                }));
            }
            let mut ctx = self.context_builder.build(user.id).await?;
            let code = self.codes.generate_code(user);

            debug!("Generated code: {:?}", code);
            ctx.send_notification(&format!("Код подтверждения: {}", code.code))
                .await;

            return Ok(tonic::Response::new(SendVerificationCodeResponse {
                left_time: None,
                error: None,
            }));
        } else {
            sleep(std::time::Duration::from_secs(1)).await;
            return Ok(tonic::Response::new(SendVerificationCodeResponse {
                left_time: None,
                error: Some(SendVerificationCodeError::VUserNotFound as i32),
            }));
        }
    }

    async fn verify_code(
        &self,
        request: tonic::Request<VerifyCodeRequest>,
    ) -> Result<tonic::Response<VerifyCodeResponse>, tonic::Status> {
        let request = request.into_inner();
        let phone_number = PhoneNumber::from(request.phone_number.as_str());
        let result = self.codes.auth(&phone_number, request.code);
        match result {
            AuthResult::Success(id) => {
                let jwt = self
                    .jwt
                    .make_jwt(Claims::new(id))
                    .map_err(|_| tonic::Status::internal("Failed to create JWT"))?;

                Ok(tonic::Response::new(VerifyCodeResponse {
                    error: None,
                    token: Some(jwt.key),
                }))
            }
            AuthResult::InvalidPhone => {
                self.codes.gc();
                Ok(tonic::Response::new(VerifyCodeResponse {
                    error: Some(VerifyCodeError::InvalidPhone as i32),
                    token: None,
                }))
            }
            AuthResult::InvalidCode => {
                self.codes.gc();
                Ok(tonic::Response::new(VerifyCodeResponse {
                    error: Some(VerifyCodeError::InvalidCode as i32),
                    token: None,
                }))
            }
            AuthResult::Expired => {
                self.codes.gc();
                Ok(tonic::Response::new(VerifyCodeResponse {
                    error: Some(VerifyCodeError::Expired as i32),
                    token: None,
                }))
            }
            AuthResult::TooManyAttempts => {
                self.codes.gc();
                Ok(tonic::Response::new(VerifyCodeResponse {
                    error: Some(VerifyCodeError::TooManyAttempts as i32),
                    token: None,
                }))
            }
        }
    }
}
