use crate::utils::*;

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

pub async fn get_handler(ctx: &Context, msg: &Message) -> Result<Arc<Mutex<Call>>, anyhow::Error> {
    let guild = msg
        .guild(&ctx.cache)
        .ok_or_else(|| anyhow!("Couldn't get guild id"))?;
    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Couldn't start manager"))?;
    manager
        .get(guild.id)
        .ok_or_else(|| anyhow!("Not currently in a channel"))
}