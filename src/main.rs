mod comandos;
mod music_controller;
mod admin;
mod utils;

use admin::*;
use music_controller::*;
use comandos::*;
use std::env;
use serenity::model::gateway::GatewayIntents;
use serenity::async_trait;
use serenity::framework::standard::{
    macros::group,
    StandardFramework,
};
use serenity::model::event::ResumedEvent;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use songbird::SerenityInit;

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
tuplas, vectores, while, let_else,
stop, play, resume, pause,
skip, mute, unmute, queue,
random, test, test_2, test_3,
ban, unban,
)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} Está Conectado!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token ERR"); // Reemplaza esto con el token de tu bot
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$").case_insensitivity(true)) // Establece el prefijo del bot como "$" y que no sea sensible a mayúsculas
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

}
