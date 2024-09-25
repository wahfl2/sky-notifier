use std::{collections::HashMap, ops::{Deref, DerefMut}};

use futures_util::lock::Mutex;

#[derive(Default)]
pub struct CtxData {
    pub request_client: reqwest::Client,
    pub hypixel_api_key: String,
    pub discord_to_mc: Mutex<TrackMut<HashMap<u64, McPlayer>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McPlayer {
    pub username: String,
    pub uuid: uuid::Uuid,
}

impl McPlayer {
    pub fn new(username: String, uuid: uuid::Uuid) -> Self {
        Self { username, uuid }
    }
}

#[derive(Default)]
pub struct TrackMut<T> {
    inner: T,
    pub mutated: bool,
}

impl<T> Deref for TrackMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TrackMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutated = true;
        &mut self.inner
    }
}


pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, CtxData, Error>;