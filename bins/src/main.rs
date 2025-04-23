use std::sync::Arc;

use env::Env;
use eyre::Context;
use services::Services;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let env = Env::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().parse_lossy("debug,teloxide=error,hyper=error,reqwest=warn,tonic_web=warn,h2=warn"))
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .pretty()
        .init();

    color_eyre::install()?;
    info!("connecting to mongo");
    let storage = storage::Storage::new(env.mongo_url())
        .await
        .context("Failed to create storage")?;
    info!("creating ledger");
    let ledger = Arc::new(Services::new(storage, env.clone()));
    info!("Starting bot...");
    let bot: bot_main::BotApp = bot_main::BotApp::new(env);
    info!("Starting mini app...");
    mini_app_main::spawn(ledger.clone(), bot.clone())?;

    info!("Starting background process...");
    bg_process::start(ledger.clone(), bot.clone()).await?;
    info!("Starting bot...");
    bot.start(ledger).await?;

    Ok(())
}
