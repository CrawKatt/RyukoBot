use super::dependencies::{Data, Error};

/// # Panics
/// if the bot fails to start or not have a Discord token
pub async fn err_handler(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}");
            }
        }
    }
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            use chrono::Local;
            let current_time = Local::now();
            let error_msg = format!("[{}] Error: {}\n", current_time.format("%Y-%m-%d %H:%M:%S"), format_args!($($arg)*));

            // Imprimir en la consola
            eprintln!("{}", error_msg);

            // Guardar en el archivo de log
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
            {
                if let Err(err) = write!(file, "{}", error_msg) {
                    eprintln!("Failed to write to log file: {}", err);
                }
            } else {
                eprintln!("Failed to open log file.");
            }
        }
    };
}