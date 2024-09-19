use std::collections::HashMap;

use futures_util::lock::Mutex;

#[derive(Default)]
pub struct CtxData {
    pub request_client: reqwest::Client,
    pub discord_to_mc: Mutex<HashMap<u64, String>>,
}

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, CtxData, Error>;