use crate::utils::*;

#[command]
#[only_in(guilds)]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Cliente Songbird Voice colocado en la inicialización.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Error: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Desconectado del Canal de Voz").await);
    } else {
        check_msg(msg.reply(ctx, "No estás en un Canal de Voz").await);
    }

    Ok(())
}