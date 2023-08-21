use crate::log_error;
use crate::utils::dependencies::{
    CommandResult,
    Context,
    FromStr,
    User,
    autocomplete_interacts,
    random,
    OpenOptions,
    Write,
    Local,
};

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

    let random_color: u32 = random::<u32>() % 0xFFFFFF;
    let category_result = nekosbest::Category::from_str(&action);
    let random_gif = match category_result {
        Ok(category) => nekosbest::get(category).await?,
        Err(error) => {
            log_error!("Error parsing category: {error}");
            return Ok(());
        }
    };

    let anime_name = random_gif.details.try_into_gif().unwrap().anime_name;
    let author = ctx.author();

    let msg = match action.as_str() {
        "kiss" => format!("{author} besó a {user}"),
        "hug"  => format!("{author} abrazó a {user}"),
        "pat" => format!("{author} acarició a {user}"),
        "slap" => format!("{author} cacheteó a {user}"),
        "kick" => format!("{author} pateó a {user}"),
        "punch" => format!("{author} le dio un puñetazo a {user}"),
        "shoot" => format!("{author} le disparó a {user}"),
        "yeet" => format!("{author} mandó a {user} a la punta del cerro"),
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
    )
    .await?;

    Ok(())
}