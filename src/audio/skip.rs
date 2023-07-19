use crate::utils::dependencies::*;

#[poise::command(slash_command, category = "Controls")]
pub async fn skip(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        ctx.say("Canción saltada").await?;
    } else {
        ctx.say("No estoy en un canal de voz").await?;
    }

    Ok(())
}