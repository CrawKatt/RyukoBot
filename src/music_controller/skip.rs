use crate::utils::*;

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let queue = handler.queue();
    queue.skip()?;

    msg.reply(&ctx.http, "Song skipped").await?;

    Ok(())
}