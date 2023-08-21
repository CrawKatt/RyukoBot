use super::dependencies::{Data, Error, Write};
use crate::log_handle;

/// # Panics
/// if the bot fails to start or not have a Discord token
pub async fn err_handler(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            log_handle!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}");
            }
        }
    }
}

#[macro_export]
macro_rules! log_handle {
    ($($arg:tt)*) => {
        {
            // Obtener la hora actual y formatearla
            let current_time = chrono::Local::now();
            let error_msg = format!("[{}] Error: {}\n", current_time.format("%Y-%m-%d %H:%M:%S"), format_args!($($arg)*));

            // Imprimir mensaje de error en la consola
            eprintln!("{}", error_msg);

            // Guardar en el archivo de log
            let log_result = std::fs::OpenOptions::new().create(true).append(true).open("log.txt");
            if let Err(err) = &log_result {
                eprintln!("Failed to open log file: {}", err);
            }

            // Escribir en el archivo de log
            if let Ok(mut file) = log_result {
                if let Err(err) = write!(file, "{}", error_msg) {
                    eprintln!("Failed to write to log file: {}", err);
                }
            }
        }
    };
}

/// # Log_error
/// ## English
/// Log an error in the console and in the log file
/// ## Español
/// Imprime un error en la consola y en un archivo de log
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            // Obtener la hora actual y formatearla
            let current_time = Local::now();
            let error_msg = format!("[{}] Error: {}\n", current_time.format("%Y-%m-%d %H:%M:%S"), format_args!($($arg)*));

            // Imprimir mensaje de error en la consola
            eprintln!("{}", error_msg);

            // Guardar en el archivo de log
            let Ok(mut file) = OpenOptions::new().create(true).append(true).open("log.txt") else {
                eprintln!("Failed to open log file.");
                return Ok(())
            };

            // Si no falla al escribir en el archivo de log, retornar Ok(())
            let Err(err) = write!(file, "{error_msg}") else {
                return Ok(())
            };

            // Si falla al escribir en el archivo de log, imprimir el error
            eprintln!("Failed to write to log file: {}", err);
        }
    };
}