use crate::utils::dependencies::*;

pub async fn try_join(ctx: Context<'_>) -> CommandResult {
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

    let already_joined = manager.get(guild_id).is_some();
    if !already_joined {
        let _handler = manager.join(guild_id, channel_id).await;
        ctx.say(format!("Joined {}", channel_id.mention())).await?;
    }

    Ok(())
}