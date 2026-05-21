use crate::config::{get_config_path, load_config};
use crate::error::AppError;

pub fn run(name: &str, verbose: bool, quiet: bool) -> Result<(), AppError> {
    let config_path = get_config_path(name)?;

    if !config_path.exists() {
        return Err(AppError::User(format!("Flow '{}' does not exist", name)));
    }

    let config = load_config(name)?;

    if config.note.is_empty() {
        if !quiet {
            println!("(no note saved)");
        }
    } else {
        if verbose {
            println!("Note for flow '{}':", name);
        }
        println!("{}", config.note);
    }

    Ok(())
}
