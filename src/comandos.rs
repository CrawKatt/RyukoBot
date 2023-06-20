use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

const HELP_MESSAGE : &str = "
Comandos Disponibles:
```
$ping: Responde con un pong!
$variables: Explica el uso de variables en Rust
```
";

const VARIABLES: &str = "
# Variables
Son un espacio en memoria que contiene un valor
### Ejemplo en Rust
```rust
let variable = 10;
```
En este ejemplo, utilizamos `let` para definir una variable, le damos un nombre que es **variable**, \
le asignamos el valor de **10** y finalizamos la sentencia con un punto y coma `;`
";

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