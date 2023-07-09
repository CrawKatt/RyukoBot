use crate::utils::*;

// Función para unirse a un canal de voz
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "No estás en un Canal de Voz para reproducir").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

// Función para reproducir un archivo de audio y añadirlo a la cola de reproducción
// (utilizar el método enqueue())
#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    join(ctx, msg).await?;
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Debes proporcionar una URL a un vídeo o audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Debes proporcionar una URL válida").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Cliente Songbird Voice colocado en la inicialización.").clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        check_msg(msg.channel_id.say(&ctx.http, "No estás en un Canal de Voz para reproducir").await);
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    let source = match Restartable::ytdl(url.clone(), true).await {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);

            check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);
            println!("Error sourcing ffmpeg {:#?}", msg);

            return Ok(());
        },
    };

    handler.enqueue_source(source.into());

    check_msg(msg.channel_id.say(&ctx.http, "Añadida canción a la playlist").await);

    Ok(())
}