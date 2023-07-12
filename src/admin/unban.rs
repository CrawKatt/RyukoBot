use crate::utils::dependencies::*;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "@username"] user: User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| anyhow!("Couldn't get guild"))?;

    let current_user = ctx.serenity_context().http.get_current_user().await?;
    let current_member = guild_id.member(&ctx.serenity_context(), current_user.id).await?;

    let has_permission = current_member
        .permissions(&ctx.serenity_context())?
        .ban_members();

    if !has_permission {
        ctx.say("No tienes permisos para desbanear a este usuario").await?;
        return Ok(());
    }

    if let Err(e) = guild_id.unban(&ctx.serenity_context(), user.id).await {
        ctx.say(format!("No se pudo desbanear al usuario: {}", e)).await?;
    } else {
        ctx.send(|f| f
            .embed(|f| f
                .color(0x00ff00)
                .description(format!("**{} ha sido desbaneado por {}!**", user, ctx.author()))
            )
        ).await?;
    }

    Ok(())
}