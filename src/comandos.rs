use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fs::read_to_string;

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
