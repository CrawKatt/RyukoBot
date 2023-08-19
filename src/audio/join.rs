use serenity::model::prelude::{ChannelId, ChannelType};
use crate::utils::dependencies::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    category_id: Option<u64>,
}

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
        ctx.say(format!("Unido a {}", channel_id.mention())).await?;
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/*                                                                        Experimental functions                                                                                 */
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: Create a function that creates a voice channel if a user joins a voice channel
pub async fn create_channel(
    ctx: Context<'_>,
    name: String,
    channel_type: ChannelType,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| anyhow!("Couldn't get guild"))?;

    let config = load_config()?;

    let Some(category) = config else {
            ctx.say("No se ha establecido un canal de voz").await?;
            return Ok(());
        };

    let category_id = ChannelId(category.into());

    let channel = guild_id
        .create_channel(&ctx.serenity_context(), |c| {
            c.name(name).kind(channel_type).category(category_id)
        }).await?;

    ctx.say(format!("Canal creado: {}", channel.mention())).await?;

    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn create(ctx: Context<'_>, name: String) -> Result<(), Error> {
    create_channel(ctx, name, ChannelType::Voice).await
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn set_voice(ctx: Context<'_>, category_id: ChannelId) -> Result<(), Error> {

    let config = Config {
        category_id: Some(category_id.0),
    };

    let serialized = serde_json::to_string(&config)?;
    save_config(serialized).await?;

    ctx.say(format!("Canal de voz establecido: {}", category_id.mention())).await?;

    Ok(())
}

async fn save_config(serialized: String) -> Result<(), Error> {
    let mut file = File::create("config.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn load_config() -> Result<Option<ChannelId>, Error> {
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;
    let category_id = config.category_id.map(ChannelId);

    Ok(category_id)
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn newvoice(
    ctx: Context<'_>,
    name: Option<String>,
) -> CommandResult {

    let guild = ctx.guild().expect("Debe ejecutarse en un servidor");

    let channel_name = name.unwrap_or_else(|| "Canal Temporal".to_string());

    let channel = guild.create_channel(ctx, |c| {
        c.name(&channel_name);
        c.kind(ChannelType::Voice);
        c.position(0);
        c
    }).await;

    ctx.send(|r| {
        r.content(format!("Canal de voz '{}' creado", channel.expect("ERROR").name()));
        r.ephemeral(true)
    }).await?;

    Ok(())
}