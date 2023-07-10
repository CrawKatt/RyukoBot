
use crate::{Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Show help docs for learning rust 
#[poise::command(prefix_command, slash_command)]
pub async fn rust(
    ctx: Context<'_>,
    #[description = "Rust concept to explain"] concept: String,
) -> Result<(), Error> {

    let path = format!("docs/{concept}.md");
    match std::fs::read_to_string(path) {
        Ok(data) => ctx.say(data).await?,
        Err(_) => ctx.say("No such doc for that concept/topic").await?
    };

    Ok(())
}


/// Show help docs for learning rust 
///
/// Enter `/explain match` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    // let num_votes = {
        // let mut hash_map = ctx.data().votes.lock().unwrap();
        // let num_votes = hash_map.entry(choice.clone()).or_default();
        // *num_votes += 1;
        // *num_votes
    // };
    let path = format!("docs/{choice}.md");
    match std::fs::read_to_string(path) {
        Ok(data) => ctx.say(data).await?,
        Err(_) => ctx.say("No such doc for that concept/topic").await?
    };



    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// ~getvotes
/// ~getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}
