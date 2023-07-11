use anyhow::anyhow;
use crate::CommandResult;
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

    let manager = songbird::get(ctx.discord())
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client placed in at initialisation."))?;

    let _handler = manager.join(guild_id, channel_id).await;

    Ok(())
}