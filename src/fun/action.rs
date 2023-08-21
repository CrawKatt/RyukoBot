use crate::log_error;
use crate::utils::dependencies::{
    Context,
    Error,
    FromStr,
    autocomplete_actions,
    random,
    OpenOptions,
    Write,
    Local
};

/// Send a gif with a status action
#[poise::command(prefix_command, slash_command)]
pub async fn act(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_actions"]
    #[description = "status to act"] action: String,
) -> Result<(), Error> {

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
        "blush" => format!("{author} Se sonroja"),
        "bored" => format!("{author} Esta aburrido"),
        "cry" => format!("{author} Esta llorando"),
        "dance" => format!("{author} Esta bailando"),
        "happy" => format!("{author} Esta feliz"),
        "laugh" => format!("{author} Se ríe a carcajadas"),
        _ => {
            log_error!("{} no es una categoría valida", action.clone());
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

// TODO: crear un autorole
/*
#[poise::command(prefix_command, track_edits, slash_command)]
async fn init(ctx: Context<'_>, channel: GuildChannel) -> CommandResult {
    let message = channel.send_message(ctx.serenity_context(), |m| {
        m.add_embed(|e| {
            e.title("Welcome to the server!")
                .description("This is a test message to see if the bot is working.")
                .timestamp(chrono::Utc::now())
        })
    }).await?;

    // TODO: añadir base de datos para el código:
    /*
    if ctx
        .data
        .database
        .get_index(&channel.guild_id)
        .await?
        .is_some()
    {
        send_application_reply(ctx, |r| {
            r.content(local_get(
                &ctx.data.translator,
                "commands_reactionroles_init_exists",
                locale,
            ))
            .ephemeral(true)
        })
        .await?;

        return Ok(());
    }
    */

    // TODO: añadir base de datos para el código:
    /*
    ctx.data
        .database
        .save_index(channel.guild_id, channel.id, message.id)
        .await?;

    send_application_reply(ctx, |r| {
        r.content(local_get(
            &ctx.data.translator,
            "commands_reactionroles_init_success",
            locale,
        ))
        .ephemeral(true)
    })
    .await?;
    */

    Ok(())
}
*/