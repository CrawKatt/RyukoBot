mod comandos;
mod info_consts;

use comandos::*;

use std::env;
use serenity::model::gateway::GatewayIntents;
use serenity::async_trait;
use serenity::framework::standard::{
    macros::group,
    StandardFramework,
};
use serenity::prelude::*;

#[group]
#[commands(
ping, help, variables, array,
borrowing, closures, condicionales, constantes,
enums, for, funciones, generics,
if_let, iterators, lifetimes, loop,
macros, match, metodos, modulos,
operadores, option, ownership, result,
return, scopes, shadowing, slices,
strings, struct, tipos_de_datos, traits,
tuplas, vectores, while,
)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token ERR"); // Reemplaza esto con el token de tu bot
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$")) // Establece el prefijo del bot como "$"
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

}
