use crate::utils::dependencies::{
    CommandResult,
    Context,
    FromStr,
    User,
    autocomplete_interacts,
    random,
    OpenOptions,
    Write,
};

use log;
use crate::log_error;

/// Interact with other users with a gif
///
/// Example:
/// ```
/// /interact greet @user
/// ```
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn interact(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_interacts"]
    #[description = "action to do with gif"]
    action: String,

    #[description = "@username"]
    user: User,

) -> CommandResult {

    let category_result = nekosbest::Category::from_str(&action);
    let random_gif = match category_result {
        Ok(category) => nekosbest::get(category).await?,
        Err(error) => {
            log_error!("Error parsing category: {error}");
            return Ok(());
        }
    };

    let anime_name = random_gif.details.try_into_gif().unwrap().anime_name;

    let random_color: u32 = random::<u32>() % 0xFFFFFF;
    let msg = match action.as_str() {
        "kiss" => format!("{} besó a {}", ctx.author(), user),
        "hug" => format!("{} abrazó a {}", ctx.author(), user),
        "pat" => format!("{} acarició a {}", ctx.author(), user),
        "slap" => format!("{} cacheteó a {}", ctx.author(), user),
        "kick" => format!("{} pateó a {}", ctx.author(), user),
        "punch" => format!("{} le dio un puñetazo a {}", ctx.author(), user),
        "shoot" => format!("{} le disparó a {}", ctx.author(), user),
        "yeet" => format!("{} mandó a {} a la punta del cerro", ctx.author(), user),
        _ => {
            log::info!("{} no es una categoria valida", action.clone());
            return Ok(())
        }
    };

    ctx.send(|f| f
        .embed(|f| f
            .color(random_color)
            .description(msg)
            .footer(|f| f.text(format!("Anime: {anime_name}")))
            .image(random_gif.url)
        )
    ).await?;

    Ok(())
}