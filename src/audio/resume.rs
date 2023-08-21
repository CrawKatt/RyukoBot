use crate::utils::dependencies::*;

#[poise::command(slash_command, category = "Controls")]
/// Resume the current track
pub async fn resume(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.say("No estoy en un canal de voz").await?;
        return Ok(())
    };

    let handler = handler_lock.lock().await;
    let queue = handler.queue();
    let _ = queue.resume();

    let Some(track) = queue.current() else {
        ctx.say("No hay canciones en la cola").await?;
        return Ok(())
    };

    ctx.say(format!("Reanudado {:?}", track.metadata().title)).await?;

    Ok(())
}