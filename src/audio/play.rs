pub use crate::utils::dependencies::*;

#[poise::command(slash_command, category = "Enqueue")]
/// Add a song to queue from the given URL.
/// TODO: Add support for searching YouTube
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL of song to play"] url: String,
) -> CommandResult {
    try_join(ctx).await?;
    play_common(ctx, url, false).await
}

async fn play_common(ctx: Context<'_>, term: String, url: bool) -> CommandResult {
    ctx.defer().await?;
    if url && !term.starts_with("http") {
        ctx.say("Argument must be a valid URL").await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let manager = songbird::get(ctx
        .serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;

    let source = if url {
        songbird::ffmpeg(&term).await?
    } else {
        songbird::ytdl(&term).await?
    };

    let call = manager
        .get(guild_id)
        .ok_or_else(|| anyhow!("Not in a voice channel"))?;

    let mut handler = call
        .lock()
        .await;

    handler.enqueue_source(source);
    let already_playing = handler
        .queue()
        .current()
        .is_some();

    if !already_playing {
        ctx.say(format!("Añadido a la cola: {}", term)).await?;
    }
    ctx.say(format!("Reproduciendo: {}", term)).await?;

    Ok(())
}