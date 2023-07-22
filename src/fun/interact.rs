use crate::utils::dependencies::*;

/// Interact with other users with a gif
///
/// Example:
/// ```
/// /interact greet @user
/// ```
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn interact(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_actions"]
    #[description = "action to do with gif"]
    action: String,

    #[description = "@username"]
    user: User,

) -> CommandResult {

    let random_gif = nekosbest::get(nekosbest::Category::from_str(&action).unwrap()).await?;
    let anime_name = random_gif.details.try_into_gif().unwrap().anime_name;

    let random_color: u32 = random::<u32>() % 0xFFFFFF;
    let msg = match action.as_str() {
        "kiss" => format!("{} beso a {}", ctx.author(), user),
        "hug" => format!("{} abrazo a {}", ctx.author(), user),
        "pat" => format!("{} acaricio a {}", ctx.author(), user),
        "slap" => format!("{} cacheteo a {}", ctx.author(), user),
        "kick" => format!("{} pateo a {}", ctx.author(), user),
        "punch" => format!("{} le dio un puñetazo a {}", ctx.author(), user),
        "shoot" => format!("{} le disparo a {}", ctx.author(), user),
        _ => {
            println!("{} no es una categoria valida", action.clone());
            return Ok(())
        }
    };

    ctx.send(|f| f
        .embed(|f| f
            .color(random_color)
            .description(msg)
            .footer(|f| f.text(format!("Anime: {}", anime_name)))
            .image(random_gif.url)
        )
    ).await?;

    Ok(())
}