use std::collections::{hash_map::Entry, HashMap};

use anyhow::anyhow;
use poise::{serenity_prelude::{self as serenity, CreateButton, CreateEmbed}, CreateReply};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{extensions::{ContextEx, CreateReplyEx}, responses, types::{Context, Error, McPlayer}};

const MOJANG_API: &str = "https://api.mojang.com";
const HYPIXEL_API: &str = "https://api.hypixel.net";

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
    let res = ctx.data().request_client.get(
        format!("{MOJANG_API}/users/profiles/minecraft/{username}")
    )
    .send()
    .await?
    .json::<responses::mojang::Profile>()
    .await?;

    if let Some(msg) = res.error_message {
        ctx.send(CreateReply::default().embed(
            CreateEmbed::new().description(
                format!("Link unsuccessful: `{}`.", msg)
            )
        )).await?;

    } else {
        let uuid = res.id.unwrap();
        let new_player = McPlayer::new(username.clone(), Uuid::try_parse(&uuid).expect("Failed to parse uuid"));

        let mut map = ctx.data().discord_to_mc.lock().await;
        let entry = map.entry(ctx.author().id.get());

        if let Entry::Occupied(mut e) = entry {
            // Previously linked

            let old_player = e.get();

            if old_player.uuid == new_player.uuid {
                let reply = CreateReply::default()
                    .embed(CreateEmbed::new().description(
                        format!("You are already linked to **{username}**!")
                    ));

                ctx.send(reply).await?;
                return Ok(())
            }

            let mut reply = CreateReply::default()
                .embed(CreateEmbed::new().description(
                    format!("Are you sure you want to unlink from **{}** and link to **{username}** instead?", old_player.username)
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
                    e.insert(new_player);
                    new_msg = format!("Successfully linked you to **{username}**.");
                } else {
                    new_msg = format!("You will stay linked to **{}**.", old_player.username);
                };

                reply = reply.embed_replace(CreateEmbed::new().description(new_msg));
            }

            sent_reply.edit(ctx, reply).await?;

            if let Some(interaction) = interaction {
                interaction.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
            }

        } else if let Entry::Vacant(e) = entry {
            // User has never linked

            e.insert(new_player);

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

async fn assure_player(
    ctx: &Context<'_>,
    username: Option<String>,
) -> Result<McPlayer, Error> {
    let player_op = ctx.mc_player().await;

    if let Some(player) = player_op {
        return Ok(player)
    }

    if username.is_none() {
        anyhow::bail!("You are not linked to a Minecraft account")
    }

    let username = username.unwrap();
    let res = ctx.data().request_client.get(
        format!("{MOJANG_API}/users/profiles/minecraft/{username}")
    )
    .send()
    .await?
    .json::<responses::mojang::Profile>()
    .await?;
    
    if let Some(error_msg) = res.error_message {
        anyhow::bail!(error_msg)
    }

    let uuid = res.id.unwrap();
    Ok(McPlayer::new(username, Uuid::try_parse(&uuid).unwrap()))
}

#[poise::command(slash_command)]
pub async fn debug_dump(
    ctx: Context<'_>,
    #[description = "Your Minecraft username"] username: Option<String>,
) -> Result<(), Error> {
    let player_res = assure_player(&ctx, username).await;

    // TODO: bals
    
    let request = ctx.data().request_client.request(
        Method::GET, 
        format!("{HYPIXEL_API}/v2/skyblock/profile"),
    )
    .header("ApiKey", &ctx.data().hypixel_api_key)
    .query(&[("uuid", )]);

    let res = request
        .send().await?
        .json::<HashMap<String, String>>().await?;

    println!("{res:?}");

    Ok(())
}