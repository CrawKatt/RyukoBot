use poise::serenity_prelude;
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

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let pages = &[
        "Página 1",
        "Página 2",
        "Página 3",
        "Página 4",
    ];

    poise::builtins::paginate(ctx, pages).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, reuse_response)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
    ctx.send(|b| {
        b.content("message 1")
            .embed(|b| b.description("embed 1").image(image_url))
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| {
                        b.label("button 1")
                            .style(serenity_prelude::ButtonStyle::Primary)
                            .custom_id(1)
                    });
                    b.create_button(|b| {
                        b.label("button 2")
                            .style(serenity_prelude::ButtonStyle::Danger)
                            .custom_id(2)
                    });
                    b.create_button(|b| {
                        b.label("button 3")
                            .style(serenity_prelude::ButtonStyle::Secondary)
                            .custom_id(3)
                    });
                    b.create_button(|b| {
                        b.label("button 4")
                            .style(serenity_prelude::ButtonStyle::Success)
                            .custom_id(4)
                    })
                })
            })
    }).await?;

    Ok(())
}