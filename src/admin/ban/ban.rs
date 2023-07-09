use crate::music_controller::utils::*;

#[command]
pub async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    // Verificar si el autor del mensaje tiene permisos de baneo
    let has_ban_permission = msg
        .member(&ctx.http)
        .await
        .map(|member| {
            let permissions = member.permissions(&ctx.cache);
            permissions.map_or(false, |p| p.ban_members())
        })
        .unwrap_or(false);

    // Si el autor no tiene el rol requerido, mostrar un mensaje de error
    if !has_ban_permission {
        msg.channel_id
            .say(&ctx.http, "No tienes los permisos necesarios para usar este comando.")
            .await?;
        return Ok(());
    }
    // Obtén el mencionado usuario del mensaje
    let user_id = match msg.mentions.get(0) {
        Some(user) => user.id,
        None => {
            msg.channel_id.say(&ctx.http, "¡No se mencionó a ningún usuario!").await?;
            return Ok(());
        }
    };

    // Obtén el miembro correspondiente al usuario mencionado
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            msg.channel_id.say(&ctx.http, "¡No se pudo obtener información del servidor!").await?;
            return Ok(());
        }
    };

    let member = match guild.member(&ctx.http, user_id).await {
        Ok(member) => member,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "¡No se pudo encontrar al usuario en el servidor!").await?;
            return Ok(());
        }
    };

    let reason = msg.content.splitn(3, ' ').nth(2).unwrap_or("Razón no especificada");
    // Banea al usuario
    let Err(error) = member.ban_with_reason(&ctx.http, 0, reason).await else {
        msg.channel_id.say(&ctx.http, format!("**¡Se baneó al usuario** {}! \n**Motivo:** *{}*", member.user.tag(), reason)).await?;
        return Ok(());
    };

    msg.channel_id.say(&ctx.http, format!("¡No se pudo banear al usuario! Error: {:?}", error)).await?;

    Ok(())
}