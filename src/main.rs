use commands::*;
use poise::serenity_prelude as serenity;
use types::{Error, CtxData};

mod commands;
mod extensions;
mod types;

const COMMANDS: &[fn() -> poise::Command<CtxData, Error>] = &[
    ping,
    link
];

async fn on_error(error: poise::FrameworkError<'_, CtxData, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error);
        }

        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }

        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = COMMANDS.iter().map(|f| f()).collect();

    let options = poise::FrameworkOptions {
        commands,
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(CtxData::default())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
