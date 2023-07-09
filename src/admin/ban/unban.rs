use crate::utils::*;

#[command]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    // Verificar si el autor del mensaje tiene permisos de desbaneo
    let has_unban_permission = msg
        .member(&ctx.http)
        .await
        .map(|member| {
            let permissions = member.permissions(&ctx.cache);
            permissions.map_or(false, |p| p.moderate_members())
        }).unwrap_or(false);

    // Si el autor no tiene el rol requerido, mostrar un mensaje de error
    if !has_unban_permission {
        msg.channel_id.say(&ctx.http, "No tienes los permisos necesarios para usar este comando.").await?;
        return Ok(());
    }

    // Obtén el ID de usuario del mensaje
    let user_id = match msg.content.split_whitespace().nth(1) {
        Some(id) => id.parse::<u64>().unwrap(),
        None => {
            msg.channel_id.say(&ctx.http, "No se especificó el ID de usuario a desbanear.").await?;
            return Ok(());
        }
    };

    // Obtén el servidor del mensaje
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(&ctx.http, "No se pudo obtener información del servidor.").await?;
            return Ok(());
        }
    };

    let reason = msg.content.splitn(3, ' ').nth(2).unwrap_or("Razón no especificada");

    // Desbanea al usuario
    let Err(error) = guild.unban(&ctx.http, user_id).await else {
        msg.channel_id.say(&ctx.http, format!("**Se desbaneó al usuario con ID:** {}. \nMotivo: *{}*", user_id, reason)).await?;
        return Ok(());
    };
    msg.channel_id.say(&ctx.http, format!("No se pudo desbanear al usuario. Error: {:?}", error)).await?;

    Ok(())
}