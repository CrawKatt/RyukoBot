pub use crate::utils::dependencies::*;

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
    #[autocomplete = "autocomplete_concepts"]
    #[description = "Rust concept to explain"] concept: String,

) -> Result<(), Error> {

    let path = format!("docs/{concept}.md");
    match std::fs::read_to_string(path) {
        Ok(data) => ctx.say(data).await?,
        Err(_) => ctx.say("No such doc for that concept/topic").await?
    };

    Ok(())
}

