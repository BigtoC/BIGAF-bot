mod constant;
mod contracts;
mod strategies;

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting BIGAF-bot...");

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| constant::RPC_URL.to_string());

    strategies::vault::execute_strategy(&rpc_url, &private_key).await?;

    info!("Bot execution finished.");
    Ok(())
}
