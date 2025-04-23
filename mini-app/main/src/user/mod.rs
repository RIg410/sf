use crate::{
    adapters::{ToModel as _, ToView as _},
    ctx::ContextBuilder,
    pb::{
        user::UserView,
        users::{users_service_server::UsersService, UserRequest},
    },
};
use model::rights::Rule;
use tonic::async_trait;
use tracing::warn;

mod map;

#[derive(Clone)]
pub struct UserServer {
    context_builder: ContextBuilder,
}

impl UserServer {
    pub fn new(context_builder: ContextBuilder) -> Self {
        UserServer { context_builder }
    }
}

#[async_trait]
impl UsersService for UserServer {
    async fn get(
        &self,
        request: tonic::Request<UserRequest>,
    ) -> std::result::Result<tonic::Response<UserView>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        let user_id = request
            .into_inner()
            .id
            .map(|id| id.to_model())
            .transpose()?
            .unwrap_or(ctx.me.id);

        let is_me = ctx.is_me(user_id);

        if !(is_me || ctx.has_right(Rule::ViewUsers)) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let user = ctx
            .services
            .get_user(&mut ctx.session, user_id)
            .await
            .map_err(|err| {
                warn!("failed to get user: {}", err);
                tonic::Status::internal("failed to get user")
            })?;

        let user = if is_me {
            user.to_view(&())
        } else {
            user.to_view(&ctx.me.rights)
        };
        Ok(tonic::Response::new(user))
    }
}
