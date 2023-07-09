use crate::utils::*;

#[command]
#[aliases(q)]
/// Show song queue
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let current_queue = handler.queue().current_queue();

    let text = if current_queue.is_empty() {
        "\nNo hay canciones en la cola de reproducción".into()
    } else {
        let mut text = String::new();
        for (i, track) in current_queue.iter().enumerate() {
            if let Some(title) = &track.metadata().title {
                text.push_str(&format!("\n{}. {}", i + 1, title));
            }
        }
        text
    };

    msg.channel_id.say(&ctx.http, format!("# Cola de Reproducción {}", text)).await?;
    Ok(())
}