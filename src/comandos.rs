use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::info_consts::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}

#[command]
async fn variables(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, VARIABLES).await?;

    Ok(())
}

#[command]
async fn array(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ARRAYS).await?;

    Ok(())
}

#[command]
async fn borrowing(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, BORROWING).await?;

    Ok(())
}

#[command]
async fn closures(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, CLOSURES).await?;

    Ok(())
}

#[command]
async fn condicionales(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, CONDICIONALES).await?;

    Ok(())
}

#[command]
async fn constantes(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, CONSTANTES).await?;

    Ok(())
}

#[command]
async fn enums(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ENUMS).await?;

    Ok(())
}

#[command]
async fn r#for(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, FOR).await?;

    Ok(())
}

#[command]
async fn funciones(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, FUNCIONES).await?;

    Ok(())
}

#[command]
async fn generics(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, GENERICS).await?;

    Ok(())
}

#[command]
async fn if_let(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, IF_LET).await?;

    Ok(())
}

#[command]
async fn iterators(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ITERATORS).await?;

    Ok(())
}

#[command]
async fn lifetimes(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, LIFETIMES).await?;

    Ok(())
}

#[command]
async fn r#loop(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, LOOP).await?;

    Ok(())
}

#[command]
async fn macros(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, MACROS).await?;

    Ok(())
}

#[command]
async fn r#match(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, MATCH).await?;

    Ok(())
}

#[command]
async fn metodos(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, METODOS).await?;

    Ok(())
}

#[command]
async fn modulos(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, MODULOS).await?;

    Ok(())
}

#[command]
async fn operadores(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, OPERADORES).await?;

    Ok(())
}

#[command]
async fn option(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, OPTION).await?;

    Ok(())
}

#[command]
async fn ownership(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, OWNERSHIP).await?;

    Ok(())
}