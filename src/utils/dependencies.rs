pub use poise::serenity_prelude::{self as serenity_2, futures};
pub use futures::{Stream, StreamExt};
pub use serde::{Deserialize, Serialize};
pub use serenity::async_trait;
pub use serenity::model::prelude::User;
pub use rand::prelude::StdRng;
pub use rand::SeedableRng;
pub use crate::fun::*;
pub use crate::{Context, Error};
pub use rand::{Rng, random};
pub use songbird::SerenityInit;

#[derive(Deserialize, Serialize, Debug)]
pub struct Original {
    pub url: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub original: Original
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Gif {
    pub images: Data,
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