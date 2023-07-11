pub use crate::utils::dependencies::*;

// Crea un comando join para que el bot se una al canal de voz
#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or_else(|| anyhow!("You're not in a voice channel"))?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client placed in at initialisation."))?;

    let _handler = manager.join(guild_id, channel_id).await;

    ctx.say(format!("Joined {}", channel_id.mention())).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;

    if let Some(call) = manager.get(guild_id) {
        call.lock().await.leave().await?;
    }

    ctx.say("Left voice channel").await?;

    Ok(())
}

#[poise::command(slash_command, category = "Enqueue")]
/// Add a song to queue from the given URL.
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL of song to play"] url: String,
) -> CommandResult {
    play_common(ctx, url, false).await
}

async fn play_common(ctx: Context<'_>, term: String, url: bool) -> CommandResult {
    ctx.defer().await?;
    if url && !term.starts_with("http") {
        ctx.say("Argument must be a valid URL").await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let manager = songbird::get(ctx.serenity_context()).await.ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;
    let source = if url {
        songbird::ffmpeg(&term).await?
    } else {
        songbird::ytdl(&term).await?
    };
    let call = manager.get(guild_id).ok_or_else(|| anyhow!("Not in a voice channel"))?;
    let mut handler = call.lock().await;
    handler.enqueue_source(source);
    ctx.say(format!("Enqueued {}", term)).await?;

    Ok(())
}