use commands::*;
use poise::serenity_prelude as serenity;
use types::{Context, CtxData, Error};

mod commands;
mod constants;
mod extensions;
mod types;
mod responses;

const COMMANDS: &[fn() -> poise::Command<CtxData, Error>] = &[
    debug_dump,
    
    link,
    ping,
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

async fn post_command(ctx: Context<'_>) {
    let data = ctx.data();
    let mut map = data.discord_to_mc.lock().await;

    if map.mutated {
        map.mutated = false;

        // serde_json::to_writer_pretty(writer, &**map);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("missing DISCORD_BOT_TOKEN");
    let hypixel_api_key = std::env::var("HYPIXEL_API_KEY").expect("missing HYPIXEL_API_KEY");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = COMMANDS.iter().map(|f| f()).collect();

    let options = poise::FrameworkOptions {
        commands,
        on_error: |error| Box::pin(on_error(error)),
        post_command: |ctx| Box::pin(post_command(ctx)),
        ..Default::default()
    };

    let context_data = CtxData {
        hypixel_api_key,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(context_data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
