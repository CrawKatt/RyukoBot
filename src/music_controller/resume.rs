use crate::utils::*;

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let queue = handler.queue();
    queue.resume()?;

    msg.reply(&ctx.http, "Reanudando reproducción").await?;

    Ok(())
}