use utils::dependencies::{
    Data, Duration, SerenityInit, act, ban, create, err_handler, help,
    interact, leave, newvoice, paginate, pause, ping, play, poise_serenity,
    resume, rust, set_voice, skip, stop, test, unban, var
};

pub mod commands;
pub mod utils;
pub mod fun;
pub mod audio;
pub mod admin;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            help(), act(), interact(),
            rust(), leave(), play(),
            pause(), resume(), stop(),
            skip(), ban(), unban(),
            paginate(), test(), create(),
            set_voice(), newvoice(), ping(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("$".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(err_handler(error)),
        /// This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        /// This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        /// Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        /// Enforce command checks even for owners (enforced by default)
        /// Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .client_settings(|b| b.register_songbird())
        .token(
            var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    client: Default::default(),
                })
            })
        })
        .options(options)
        .intents(
            poise_serenity::GatewayIntents::non_privileged() | poise_serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}