use crate::utils::*;

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let _ = handler.queue().pause();

        check_msg(msg.channel_id.say(&ctx.http, "Paused song").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to pause").await);
    }

    Ok(())
}