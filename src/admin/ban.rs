use crate::utils::dependencies::{
    Context,
    Error,
    User,
    anyhow
};

/// Ban a user from the server
///
/// Example:
/// ```
/// /ban @user
/// ```
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "@username"] user: User, razon: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| anyhow!("Couldn't get guild"))?;
    let member = guild_id.member(&ctx.serenity_context(), user.id).await?;
    let author = ctx.author();

    let has_permission = member
        .permissions(ctx
        .serenity_context())?
        .ban_members();

    if !has_permission {
        ctx.say("No tienes permisos para banear a este usuario").await?;
        return Ok(());
    }

    if let Err(e) = member.ban_with_reason(&ctx.serenity_context(), 0, "Banned by bot").await {
        ctx.say(format!("Failed to ban user: {e}")).await?;
    } else {
        let razon =  razon.map_or_else(|| "No especificada".to_string(), |reason| reason);

        ctx.send(|f| f
            .embed(|f| f
                .color(0xff0000)
                .description(format!("##### **{user} ha sido baneado por {author}!** \n##### **Motivo:** *{razon}*"))
            )).await?;
    }

    Ok(())
}