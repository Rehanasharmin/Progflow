use std::process::Command;

use crate::config::get_config_path;
use crate::error::AppError;
use crate::platform::get_editor;

pub fn run(name: &str) -> Result<(), AppError> {
    let config_path = get_config_path(name)?;

    if !config_path.exists() {
        return Err(AppError::User(format!(
            "Config file does not exist: {}",
            config_path.display()
        )));
    }

    let editor = get_editor()
        .ok_or_else(|| AppError::User("No editor set. Set $EDITOR or $VISUAL".to_string()))?;

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", editor, config_path.display()))
        .spawn()?
        .wait()
        .map_err(|e| AppError::Io("editor".to_string(), e))?;

    if !status.success() {
        return Err(AppError::User(format!(
            "Editor exited with non-zero: {:?}",
            status.code()
        )));
    }

    Ok(())
}
