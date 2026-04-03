use std::io::{self, IsTerminal, Write};

use crate::config::{delete_lock_file, get_config_path};
use crate::error::AppError;

pub fn run(name: &str, force: bool) -> Result<(), AppError> {
    let config_path = get_config_path(name)?;

    if !config_path.exists() {
        return Err(AppError::User(format!("Flow '{}' does not exist", name)));
    }

    let is_interactive = io::stdin().is_terminal() && !force;

    if is_interactive {
        print!("Delete flow '{}'? [y/N]: ", name);
        io::stdout()
            .flush()
            .map_err(|e| AppError::Io("stdout".to_string(), e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;

        if input.trim().to_lowercase() != "y" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    std::fs::remove_file(&config_path)
        .map_err(|e| AppError::Io(config_path.display().to_string(), e))?;

    let _ = delete_lock_file(name);

    println!("✓ flow '{}' deleted", name);

    Ok(())
}
