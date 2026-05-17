use std::process::Command;

use crate::config::{get_config_path, load_config, save_config};
use crate::error::AppError;
use crate::platform::get_editor;

pub fn run(name: &str, set_note: Option<String>, quiet: bool) -> Result<(), AppError> {
    if let Some(note) = set_note {
        let mut config = load_config(name)?;
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        let formatted_note = format!("[{}] {}", timestamp, note.trim());
        config.note = formatted_note.clone();
        config.last_note = Some(formatted_note);
        save_config(&config)?;
        if !quiet {
            println!("✓ note updated for flow '{}'", name);
        }
        return Ok(());
    }

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
