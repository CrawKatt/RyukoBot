use crate::utils::dependencies::*;

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

    #[description = "@username"] user: serenity_2::User,
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