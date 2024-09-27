use std::{fs::File, io::{BufWriter, Write}};

use poise::{serenity_prelude as serenity, CreateReply};
use reqwest::Method;
use uuid::Uuid;

use crate::{constants::{HYPIXEL_API, MOJANG_API}, extensions::{ContextEx, CreateReplyEx}, responses, types::{Context, Error, McPlayer}};

#[poise::command(slash_command)]
pub async fn debug_dump(
    ctx: Context<'_>,
    #[description = "Your Minecraft username"] username: Option<String>,
) -> Result<(), Error> {
    let player_res = assure_player(&ctx, username).await;

    if let Err(err) = player_res {
        ctx.send(CreateReply::simple_embed(err.to_string())).await?;
        return Ok(())
    }

    let player = player_res.unwrap();

    let request = ctx.data().request_client.request(
        Method::GET, 
        format!("{HYPIXEL_API}/v2/skyblock/profiles"),
    )
    .header("API-Key", &ctx.data().hypixel_api_key)
    .query(&[("uuid", player.uuid.to_string())]);

    let res = request
        .send().await?
        .json::<serde_json::Value>().await?;

    if !res.get("success").unwrap().as_bool().unwrap() {
        println!("{res:?}");
        let cause = res.get("cause").unwrap().as_str().unwrap();
        ctx.send(CreateReply::simple_embed(format!("Something went wrong: {cause}"))).await?;
        return Ok(())
    }

    let file = File::create(format!("./dump/{}.json", player.username))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &res)?;
    writer.flush()?;

    ctx.send(CreateReply::simple_embed(
        format!("Hypixel profile data dumped to `{}.json`", player.username)
    )).await?;

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

