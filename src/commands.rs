use poise::serenity_prelude as serenity;

use crate::types::{Context, Error};

pub mod debug;
pub mod link;

pub use debug::*;
pub use link::*;

#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let ping = serenity::Timestamp::now()
        .signed_duration_since(ctx.created_at().to_utc());
    
    ctx.say(format!("Pong! `{}ms`", ping.num_milliseconds())).await?;
    Ok(())
}