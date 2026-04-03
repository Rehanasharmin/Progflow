use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use crate::config::{load_config, write_lock_file};
use crate::error::AppError;
use crate::platform::spawn_url;

pub fn run(name: &str, verbose: bool) -> Result<(), AppError> {
    let config = load_config(name)?;

    config.validate()?;

    if let Some(ref dir) = config.directory {
        let path = Path::new(dir);
        if !path.exists() {
            return Err(AppError::with_suggestion(
                &format!("Directory does not exist: {}", dir),
                "Run 'progflow edit {name}' to update the directory path",
            ));
        }
    }

    let mut pids: Vec<u32> = Vec::new();

    let work_dir = config.directory.as_deref().unwrap_or(".");

    if let Some(ref editor_cmd) = config.editor_cmd {
        if verbose {
            eprintln!("Spawning editor: {}", editor_cmd);
        }
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(editor_cmd).current_dir(work_dir);
        cmd.stdin(std::process::Stdio::null());

        apply_env(&mut cmd, &config.env);

        match cmd.spawn() {
            Ok(child) => {
                pids.push(child.id());
            }
            Err(e) => {
                eprintln!("Warning: Failed to spawn editor '{}': {}", editor_cmd, e);
            }
        }
    }

    if let Some(ref urls) = config.url_list {
        for url in urls {
            if verbose {
                eprintln!("Opening URL: {}", url);
            }
            spawn_url(url);
        }
    }

    let url_count = config.url_list.as_ref().map(|u| u.len()).unwrap_or(0);

    write_lock_file(name, pids.clone())?;

    let mut parts: Vec<String> = vec![];
    if config.editor_cmd.is_some() {
        parts.push("editor".to_string());
    }
    if url_count > 0 {
        let suffix = if url_count == 1 { "url" } else { "urls" };
        parts.push(format!("{} {}", url_count, suffix));
    }

    let summary = if parts.is_empty() {
        "started (no processes)".to_string()
    } else {
        parts.join(", ")
    };

    println!("✓ flow '{}' started — {}", name, summary);

    Ok(())
}

fn apply_env(cmd: &mut Command, env_vars: &HashMap<String, String>) {
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
}
