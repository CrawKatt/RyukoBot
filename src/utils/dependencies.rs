pub use poise::serenity_prelude::futures;
pub use poise::serenity_prelude as poise_serenity;
pub use serenity::prelude::Mentionable;
pub use anyhow::anyhow;
pub use songbird::SerenityInit;
pub use serenity::async_trait;
pub use serenity::model::prelude::User;
pub use futures::{
    Stream,
    StreamExt
};

pub use std::{
    collections::HashMap,
    env::var,
    sync::Mutex,
    time::Duration
};

pub use serde::{
    Deserialize,
    Serialize
};

pub use rand::prelude::StdRng;
pub use rand::SeedableRng;
pub use crate::fun::*;
pub use crate::Context;

pub use rand::{
    Rng,
    random
};

pub use crate::commands::*;
pub use crate::audio::*;
pub type CommandResult = Result<(), Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize, Serialize, Debug)]
pub struct Original {
    pub url: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GifData {
    pub original: Original
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Gif {
    pub images: GifData,
    pub slug: String
}

#[derive(Deserialize, Serialize)]
pub struct ResponseGiphy {
    pub data: Vec<Gif>,
}

pub async fn autocomplete_actions<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["greet", "kiss", "kick", "slap", "punch", "gun"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

pub async fn autocomplete_status<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["sleep", "playing", "boring", "angry", "happy" ])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}