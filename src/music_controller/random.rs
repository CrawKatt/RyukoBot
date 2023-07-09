use crate::utils::*;

#[command]
#[aliases(r)]
/// Shuffle the song queue
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let call = get_handler(ctx, msg).await?;
    let handler = call.lock().await;

    let current_queue = handler.queue().current_queue();
    println!("current_queue: {:#?}", current_queue);

    let text = if current_queue.is_empty() {
        "No hay canciones en la cola".into()
    } else {
        let mut current_queue = current_queue.clone();
        current_queue.shuffle(&mut rand::thread_rng()); // Mezcla la cola de canciones

        let mut text = String::new();
        for (i, track) in current_queue.iter().enumerate() {
            if let Some(title) = &track.metadata().title {
                text.push_str(&format!("{}. {}\n", i + 1, title));
            }
        }
        text
    };

    msg.channel_id.say(&ctx.http, text).await?;
    Ok(())
}