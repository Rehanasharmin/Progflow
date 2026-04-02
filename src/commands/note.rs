use crate::config::{get_config_path, load_config};
use crate::error::AppError;

pub fn run(name: &str) -> Result<(), AppError> {
    let config_path = get_config_path(name)?;

    if !config_path.exists() {
        return Err(AppError::User(format!("Flow '{}' does not exist", name)));
    }

    let config = load_config(name)?;

    if config.note.is_empty() {
        println!("(no note saved)");
    } else {
        println!("{}", config.note);
    }

    Ok(())
}
