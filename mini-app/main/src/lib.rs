use auth::AuthServer;
use bot_main::BotApp;
use eyre::Result;
use ledger::Ledger;
use pb::{
    auth::auth_service_server::AuthServiceServer, users::users_service_server::UsersServiceServer,
};
use std::sync::Arc;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tracing::debug;
use user::UserServer;

pub(crate) mod auth;
pub(crate) mod ctx;
pub(crate) mod pb;
pub(crate) mod user;
pub(crate) mod adapters;

pub fn spawn(ledger: Arc<Ledger>, bot: BotApp) -> Result<()> {
    let ctx_builder = ctx::ContextBuilder::new(ledger.clone(), bot.clone());

    tokio::spawn(async move {
        let addr = "0.0.0.0:3000".parse().unwrap();
        debug!("listening on {}", addr);
        Server::builder()
            .accept_http1(true)
            .layer(GrpcWebLayer::new())
            .add_service(AuthServiceServer::new(AuthServer::new(ctx_builder.clone())))
            .add_service(UsersServiceServer::new(UserServer::new(ctx_builder)))
            .serve(addr)
            .await
            .unwrap();
    });
    Ok(())
}
