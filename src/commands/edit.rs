use std::collections::HashMap;
use std::process::Command;

use crate::config::{get_config_path, load_config, save_config};
use crate::error::AppError;
use crate::platform::get_editor;

#[allow(clippy::too_many_arguments)]
pub fn run(
    name: &str,
    dir: Option<String>,
    editor_flag: Option<String>,
    urls: Option<String>,
    env: Option<String>,
    shell: Option<String>,
    set_note: Option<String>,
    start_commands: Option<String>,
    quiet: bool,
) -> Result<(), AppError> {
    let mut config = load_config(name)?;

    let is_updating = dir.is_some()
        || editor_flag.is_some()
        || urls.is_some()
        || env.is_some()
        || shell.is_some()
        || set_note.is_some()
        || start_commands.is_some();

    if is_updating {
        if let Some(d) = dir {
            config.directory = Some(d);
        }
        if let Some(e) = editor_flag {
            config.editor_cmd = Some(e);
        }
        if let Some(u) = urls {
            config.url_list = Some(u.split(',').map(|s| s.trim().to_string()).collect());
        }
        if let Some(s) = shell {
            config.shell = s;
        }
        if let Some(env_str) = env {
            let mut new_env = HashMap::new();
            for pair in env_str.split(',') {
                if let Some((k, v)) = pair.split_once('=') {
                    new_env.insert(k.trim().to_string(), v.trim().to_string());
                }
            }
            config.env = new_env;
        }
        if let Some(note) = set_note {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
            let formatted_note = format!("[{}] {}", timestamp, note.trim());
            config.note = formatted_note.clone();
            config.last_note = Some(formatted_note);
        }
        if let Some(sc_json) = start_commands {
            config.start_commands = serde_json::from_str(&sc_json)
                .map_err(|e| AppError::User(format!("Invalid start commands JSON: {}", e)))?;
        }

        config.validate()?;
        save_config(&config)?;

        if !quiet {
            println!("✓ flow '{}' updated", name);
        }
        return Ok(());
    }

    // Interactive edit (open in editor)
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
