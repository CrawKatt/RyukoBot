// Anyhow Dependencies
pub use anyhow::anyhow;

// Songbird Dependencies
pub use songbird::SerenityInit;

// Poise Dependencies
pub use poise::serenity_prelude::futures;
pub use poise::serenity_prelude as poise_serenity;

// Serenity Dependencies
pub use serenity::async_trait;
pub use serenity::prelude::Mentionable;
pub use serenity::model::prelude::User;

// Std Dependencies
pub use futures::{
    Stream,
    StreamExt
};

pub use std::{
    collections::HashMap,
    env::var,
    sync::Mutex,
    time::Duration,
    str::FromStr
};

// Serde Dependencies
pub use serde::{
    Deserialize,
    Serialize
};

// Rand Dependencies
pub use rand::random;

// Re-export all commands
pub use crate::commands::*;
pub use crate::audio::*;
pub use crate::admin::*;
pub use crate::fun::*;
use crate::utils::autocomplete_arguments::{AUTOCOMPLETE_ACTIONS, AUTOCOMPLETE_INTERACTS};

// Re-export all utils
pub use crate::utils::*;

// Types used by all command functions
pub type CommandResult = Result<(), Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    pub client: reqwest::Client,
}

/// Autocomplete functions for slash commands
pub async fn autocomplete_actions<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(AUTOCOMPLETE_ACTIONS)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

pub async fn autocomplete_status<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(AUTOCOMPLETE_INTERACTS)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// # Panic if docs folder not found
pub async fn autocomplete_concepts<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let Ok(docs) = std::fs::read_dir("docs") else {
        panic!("Docs not found required!!!");
    };

    let mut files = vec![];
    for entry in docs.into_iter() {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();
        let name = filename.split('.').next().unwrap();
        files.push(name.to_string());
    }

    futures::stream::iter(files)
        .filter(move |data: &String| futures::future::ready(data.starts_with(partial)))
}