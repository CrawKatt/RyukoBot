use poise::serenity_prelude::{self as serenity, futures};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

use crate::{Context, Error};
use rand::{Rng, random};

#[derive(Deserialize, Serialize, Debug)]
struct Original {
    url: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    original: Original
}

#[derive(Deserialize, Serialize, Debug)]
struct Gif {
    images: Data,
    slug: String
}

#[derive(Deserialize, Serialize)]
struct ResponseGiphy {
    data: Vec<Gif>,
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Show help docs for learning rust 
#[poise::command(prefix_command, slash_command)]
pub async fn rust(
    ctx: Context<'_>,
    #[description = "Rust concept to explain"] concept: String,
) -> Result<(), Error> {

    let path = format!("docs/{concept}.md");
    match std::fs::read_to_string(path) {
        Ok(data) => ctx.say(data).await?,
        Err(_) => ctx.say("No such doc for that concept/topic").await?
    };

    Ok(())
}

async fn autocomplete_status<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["sleep", "playing", "boring", "angry", "happy" ])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}


/// Show help docs for learning rust 
///
/// Enter `/explain match` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn act(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_status"]
    #[description = "status to act"] action: String,
) -> Result<(), Error> {

    let url = format!("https://api.giphy.com/v1/gifs/search?api_key=76BcX0eFN6wWFLz3P5CpCj7al8AcWWOK&q={action}&limit=25&offset=0&rating=g&lang=en&bundle=messaging_non_clips");
    let resp = reqwest::get(url)
        .await?
        .json::<ResponseGiphy>()
        .await?;

    let random_color: u32 = random::<u32>() % 0xFFFFFF;

    let index_random = rand::thread_rng().gen_range(0..resp.data.len());

    let random_gif = resp.data.get(index_random);

    let msg = match action.as_str() {
        "greet" => format!("{} saluda a todos", ctx.author()),
        "playing" => format!("{} esta jugando", ctx.author()),
        "angry" => format!("{} esta enojado", ctx.author()),
        "happy" => format!("{} esta feliz", ctx.author()),
        "boring" => format!("{} esta aburrido", ctx.author()),
        _ => format!("{} esta {}",  ctx.author(), action),
    };

    ctx.send(|f| f
        .content(msg)
        .embed(|f| f
            .color(random_color)
            .image(random_gif.unwrap().images.original.url.as_str())
        )
    ).await?;



    Ok(())
}

async fn autocomplete_actions<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["greet", "kiss", "kick", "slap", "punch", "gun"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// Interact with other users with a gif
///
/// Example: 
/// ```
/// /interact greet @user
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn interact(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_actions"]
    #[description = "action to do with gif"] 
    action: String,

    #[description = "@username"] user: serenity::User,
) -> Result<(), Error> {

    let url = format!("https://api.giphy.com/v1/gifs/search?api_key=76BcX0eFN6wWFLz3P5CpCj7al8AcWWOK&q={action}&limit=25&offset=0&rating=g&lang=en&bundle=messaging_non_clips");
    let resp = reqwest::get(url)
        .await?
        .json::<ResponseGiphy>()
        .await?;

    let random_color: u32 = random::<u32>() % 0xFFFFFF;

    let index_random = rand::thread_rng().gen_range(0..resp.data.len());

    let random_gif = resp.data.get(index_random);

    let msg = match action.as_str() {
        "greet" => format!("{} saluda a {}", ctx.author(), user),
        "kiss" => format!("{} beso a {}", ctx.author(), user),
        "slap" => format!("{} cachetea a {}", ctx.author(), user),
        "kick" => format!("{} pateo a {}", ctx.author(), user),
        "punch" => format!("{} le dio un puñetazo a {}", ctx.author(), user),
        "gun" => format!("{} le disparo a {}", ctx.author(), user),
        _ => format!("{} {} {}", ctx.author(), action, user),
    };

    ctx.send(|f| f
        .content(msg)
        .embed(|f| f
            .color(random_color)
            .image(random_gif.unwrap().images.original.url.as_str())
        )
    ).await?;

    Ok(())

}


