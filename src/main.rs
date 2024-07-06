mod command;
mod signal;

use std::sync::Arc;

use anyhow::{Context as _, Result};
use poise::serenity_prelude as serenity;

use crate::command::bypass_verify;
use crate::signal::wait_for_signal;

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() -> Result<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![bypass_verify()],
            ..Default::default()
        })
        .setup(move |ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let token = std::env::var("SKIP_KUN_TOKEN").context("Failed to get $SKIP_KUN_TOKEN")?;
    let intents = serenity::GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    let shard_manager = Arc::clone(&client.shard_manager);

    tokio::spawn(async move {
        wait_for_signal()
            .await
            .expect("Failed to register signal handler");

        println!("Shutting down...");
        shard_manager.shutdown_all().await;
    });

    println!("Starting...");
    client.start_autosharded().await?;

    Ok(())
}
