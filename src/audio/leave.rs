use crate::utils::dependencies::*;

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialised."))?;

    let Some(call) = manager.get(guild_id) else {
        ctx.say("No estoy en un canal de voz").await?;
        return Ok(());
    };

    call.lock().await.leave().await?;
    ctx.say("Desconexión del canal de voz").await?;

    Ok(())
}