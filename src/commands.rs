use std::collections::hash_map::Entry;

use poise::{serenity_prelude::{self as serenity, CreateButton, CreateEmbed}, CreateReply};
use serde::{Deserialize, Serialize};

use crate::{extensions::CreateReplyEx, types::{Context, Error}};

const MOJANG_API: &str = "https://api.mojang.com";

#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let ping = serenity::Timestamp::now()
        .signed_duration_since(ctx.created_at().to_utc());
    
    ctx.say(format!("Pong! `{}ms`", ping.num_milliseconds())).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "Your Minecraft username"] username: String,
) -> Result<(), Error> {

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        // name: Option<String>,
        // id: Option<String>,
        error_message: Option<String>,
        // path: Option<String>,
    }

    let res = ctx.data().request_client.get(
        format!("{MOJANG_API}/users/profiles/minecraft/{username}")
    )
    .send()
    .await?
    .json::<Response>()
    .await?;

    if let Some(msg) = res.error_message {
        ctx.send(CreateReply::default().embed(
            CreateEmbed::new().description(
                format!("Link unsuccessful: `{}`.", msg)
            )
        )).await?;

    } else {
        
        let mut map = ctx.data().discord_to_mc.lock().await;
        let entry = map.entry(ctx.author().id.get());

        if let Entry::Occupied(mut e) = entry {
            // Previously linked

            let old_username = e.get();

            if *old_username == username {
                let reply = CreateReply::default()
                    .embed(CreateEmbed::new().description(
                        format!("You are already linked to **{username}**!")
                    ));

                ctx.send(reply).await?;
                return Ok(())
            }

            let mut reply = CreateReply::default()
                .embed(CreateEmbed::new().description(
                    format!("Are you sure you want to unlink from **{old_username}** and link to **{username}** instead?")
                ))
                .button(CreateButton::new("relink_confirm")
                    .label("Yes")
                    .style(serenity::ButtonStyle::Success)
                )
                .button(CreateButton::new("relink_deny")
                    .label("No")
                    .style(serenity::ButtonStyle::Danger)
                );

            let sent_reply = ctx.send(reply.clone()).await?;
            let msg = sent_reply.message().await?;

            let interaction = msg
                .await_component_interaction(&ctx.serenity_context().shard)
                .await;

            // Remove buttons even if timeout
            reply = reply.components(Vec::new());

            if let Some(ref interaction) = interaction {
                let new_msg;

                if interaction.data.custom_id == "relink_confirm" {
                    e.insert(username.clone());
                    new_msg = format!("Successfully linked you to **{username}**.")
                } else {
                    new_msg = format!("You will stay linked to **{old_username}**.")
                };

                reply = reply.embed_replace(CreateEmbed::new().description(new_msg));
            }

            sent_reply.edit(ctx, reply).await?;

            if let Some(interaction) = interaction {
                interaction.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
            }

        } else if let Entry::Vacant(e) = entry {
            // User has never linked

            e.insert(username.clone());

            ctx.send(CreateReply::default().embed(
                CreateEmbed::new().description(
                    format!("Successfully linked you to **{username}**.")
                )
            )).await?;

        } else {
            unreachable!()
        }
    }

    Ok(())
}