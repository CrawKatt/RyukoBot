use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::{Result as SerenityResult};
use serenity::framework::standard::Args;
use std::fs::read_to_string;
use std::sync::Arc;
use anyhow::anyhow;

use serenity::{
    client::{Context},
    prelude::Mutex,
};
use songbird::{
    Call,
};

// crear comando que haga que el bot entre a un voice chat
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
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

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to unmute in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let queue = handler.queue();
    queue.skip()?;

    msg.reply(&ctx.http, "Song skipped").await?;

    Ok(())
}
/*
#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.pause();

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xa6e3a1)
                        .title(":pause_button: Paused!")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.colour(0xf38ba8)
                        .title(":warning: Not in a voice channel.")
                        .timestamp(Timestamp::now())
                })
            })
            .await?;
    }
    Ok(())
}
*/

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

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let queue = handler.queue();
    queue.resume()?;

    msg.reply(&ctx.http, "Resumed playing").await?;

    Ok(())
}

async fn get_handler(ctx: &Context, msg: &Message) -> Result<Arc<Mutex<Call>>, anyhow::Error> {
    let guild = msg
        .guild(&ctx.cache)
        .ok_or_else(|| anyhow!("Couldn't get guild id"))?;
    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Couldn't start manager"))?;
    Ok(manager
        .get(guild.id)
        .ok_or_else(|| anyhow!("Not currently in a channel"))?)
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    join(ctx, msg).await?;
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        check_msg(msg.channel_id.say(&ctx.http, "No estás en un Canal de Voz para reproducir").await);
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    let source = match songbird::ytdl(&url).await {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);

            check_msg(msg.channel_id.say(&ctx.http, format!("Error sourcing ffmpeg")).await);
            println!("Error sourcing ffmpeg {:#?}", msg);

            return Ok(());
        },
    };

    handler.enqueue_source(source);

    check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);

    Ok(())

}

/*
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        handler.play_source(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }
    */

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help = read_to_string("docs/help.md").unwrap();
    msg.channel_id.say(&ctx.http, help).await?;

    Ok(())
}

#[command]
async fn variables(ctx: &Context, msg: &Message) -> CommandResult {
    let variables = read_to_string("docs/variables.md").unwrap();
    msg.channel_id.say(&ctx.http, variables).await?;

    Ok(())
}

#[command]
async fn array(ctx: &Context, msg: &Message) -> CommandResult {
    let arrays = read_to_string("docs/arrays.md").unwrap();
    msg.channel_id.say(&ctx.http, arrays).await?;

    Ok(())
}

#[command]
async fn borrowing(ctx: &Context, msg: &Message) -> CommandResult {
    let borrowing = read_to_string("docs/borrowing.md").unwrap();
    msg.channel_id.say(&ctx.http, borrowing).await?;

    Ok(())
}

#[command]
async fn closures(ctx: &Context, msg: &Message) -> CommandResult {
    let closures = read_to_string("docs/closures.md").unwrap();
    msg.channel_id.say(&ctx.http, closures).await?;

    Ok(())
}

#[command]
async fn condicionales(ctx: &Context, msg: &Message) -> CommandResult {
    let condicionales = read_to_string("docs/condicionales.md").unwrap();
    msg.channel_id.say(&ctx.http, condicionales).await?;

    Ok(())
}

#[command]
async fn constantes(ctx: &Context, msg: &Message) -> CommandResult {
    let constantes = read_to_string("docs/constantes.md").unwrap();
    msg.channel_id.say(&ctx.http, constantes).await?;

    Ok(())
}

#[command]
async fn enums(ctx: &Context, msg: &Message) -> CommandResult {
    let enums = read_to_string("docs/enums.md").unwrap();
    msg.channel_id.say(&ctx.http, enums).await?;

    Ok(())
}

#[command]
async fn r#for(ctx: &Context, msg: &Message) -> CommandResult {
    let for_data = read_to_string("docs/for.md").unwrap();
    msg.channel_id.say(&ctx.http, for_data).await?;

    Ok(())
}

#[command]
async fn funciones(ctx: &Context, msg: &Message) -> CommandResult {
    let funciones = read_to_string("docs/funciones.md").unwrap();
    msg.channel_id.say(&ctx.http, funciones).await?;

    Ok(())
}

#[command]
async fn generics(ctx: &Context, msg: &Message) -> CommandResult {
    let generics = read_to_string("docs/generics.md").unwrap();
    msg.channel_id.say(&ctx.http, generics).await?;

    Ok(())
}

#[command]
async fn if_let(ctx: &Context, msg: &Message) -> CommandResult {
    let if_let = read_to_string("docs/if_let.md").unwrap();
    msg.channel_id.say(&ctx.http, if_let).await?;

    Ok(())
}

#[command]
async fn iterators(ctx: &Context, msg: &Message) -> CommandResult {
    let iterators = read_to_string("docs/iterators.md").unwrap();
    msg.channel_id.say(&ctx.http, iterators).await?;

    Ok(())
}

#[command]
async fn lifetimes(ctx: &Context, msg: &Message) -> CommandResult {
    let lifetimes = read_to_string("docs/lifetimes.md").unwrap();
    msg.channel_id.say(&ctx.http, lifetimes).await?;

    Ok(())
}

#[command]
async fn r#loop(ctx: &Context, msg: &Message) -> CommandResult {
    let loop_data = read_to_string("docs/loop.md").unwrap();
    msg.channel_id.say(&ctx.http, loop_data).await?;

    Ok(())
}

#[command]
async fn macros(ctx: &Context, msg: &Message) -> CommandResult {
    let macros = read_to_string("docs/macros.md").unwrap();
    msg.channel_id.say(&ctx.http, macros).await?;

    Ok(())
}

#[command]
async fn r#match(ctx: &Context, msg: &Message) -> CommandResult {
    let match_data = read_to_string("docs/match.md").unwrap();
    msg.channel_id.say(&ctx.http, match_data).await?;

    Ok(())
}

#[command]
async fn metodos(ctx: &Context, msg: &Message) -> CommandResult {
    let metodos = read_to_string("docs/metodos.md").unwrap();
    msg.channel_id.say(&ctx.http, metodos).await?;

    Ok(())
}

#[command]
async fn modulos(ctx: &Context, msg: &Message) -> CommandResult {
    let modulos = read_to_string("docs/modulos.md").unwrap();
    msg.channel_id.say(&ctx.http, modulos).await?;

    Ok(())
}

#[command]
async fn operadores(ctx: &Context, msg: &Message) -> CommandResult {
    let operadores = read_to_string("docs/operadores.md").unwrap();
    msg.channel_id.say(&ctx.http, operadores).await?;

    Ok(())
}

#[command]
async fn option(ctx: &Context, msg: &Message) -> CommandResult {
    let option = read_to_string("docs/option.md").unwrap();
    msg.channel_id.say(&ctx.http, option).await?;

    Ok(())
}

#[command]
async fn ownership(ctx: &Context, msg: &Message) -> CommandResult {
    let ownership = read_to_string("docs/ownership.md").unwrap();
    msg.channel_id.say(&ctx.http, ownership).await?;

    Ok(())
}

#[command]
async fn result(ctx: &Context, msg: &Message) -> CommandResult {
    let result = read_to_string("docs/result.md").unwrap();
    msg.channel_id.say(&ctx.http, result).await?;

    Ok(())
}

#[command]
async fn r#return(ctx: &Context, msg: &Message) -> CommandResult {
    let return_data = read_to_string("docs/return.md").unwrap();
    msg.channel_id.say(&ctx.http, return_data).await?;

    Ok(())
}

#[command]
async fn scopes(ctx: &Context, msg: &Message) -> CommandResult {
    let scopes = read_to_string("docs/scopes.md").unwrap();
    msg.channel_id.say(&ctx.http, scopes).await?;

    Ok(())
}

#[command]
async fn shadowing(ctx: &Context, msg: &Message) -> CommandResult {
    let shadowing: String = read_to_string("docs/shadowing.md").unwrap();
    msg.channel_id.say(&ctx.http, shadowing).await?;

    Ok(())
}

#[command]
async fn slices(ctx: &Context, msg: &Message) -> CommandResult {
    let slices = read_to_string("docs/slices.md").unwrap();
    msg.channel_id.say(&ctx.http, slices).await?;

    Ok(())
}

#[command]
async fn strings(ctx: &Context, msg: &Message) -> CommandResult {
    let strings = read_to_string("docs/strings.md").unwrap();
    msg.channel_id.say(&ctx.http, strings).await?;

    Ok(())
}

#[command]
async fn r#struct(ctx: &Context, msg: &Message) -> CommandResult {
    let struct_data = read_to_string("docs/struct.md").unwrap();
    msg.channel_id.say(&ctx.http, struct_data).await?;

    Ok(())
}

#[command]
async fn tipos_de_datos(ctx: &Context, msg: &Message) -> CommandResult {
    let tipos_de_datos = read_to_string("docs/tipos_de_datos.md").unwrap();
    msg.channel_id.say(&ctx.http, tipos_de_datos).await?;

    Ok(())
}

#[command]
async fn traits(ctx: &Context, msg: &Message) -> CommandResult {
    let traits = read_to_string("docs/traits.md").unwrap();
    msg.channel_id.say(&ctx.http, traits).await?;

    Ok(())
}

#[command]
async fn tuplas(ctx: &Context, msg: &Message) -> CommandResult {
    let tuplas = read_to_string("docs/tuplas.md").unwrap();
    msg.channel_id.say(&ctx.http, tuplas).await?;

    Ok(())
}

#[command]
async fn vectores(ctx: &Context, msg: &Message) -> CommandResult {
    let vectores = read_to_string("docs/vectores.md").unwrap();
    msg.channel_id.say(&ctx.http, vectores).await?;

    Ok(())
}

#[command]
async fn r#while(ctx: &Context, msg: &Message) -> CommandResult {
    let while_data = read_to_string("docs/while.md").unwrap();
    msg.channel_id.say(&ctx.http, while_data).await?;

    Ok(())
}

#[command]
async fn let_else(ctx: &Context, msg: &Message) -> CommandResult {
    let let_else = read_to_string("docs/let_else.md").unwrap();
    msg.channel_id.say(&ctx.http, let_else).await?;

    Ok(())
}
