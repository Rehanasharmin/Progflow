use std::collections::HashMap;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crate::config::{load_config, save_config, write_lock_file};
use crate::error::AppError;
use crate::platform::{get_editor, spawn_url};

pub fn run(
    name: &str,
    skip_url_check: bool,
    edit_note: bool,
    verbose: bool,
    quiet: bool,
) -> Result<(), AppError> {
    let mut config = load_config(name)?;

    config.validate()?;

    if let Some(ref dir) = config.directory {
        let path = Path::new(dir);
        if !path.exists() {
            return Err(AppError::with_suggestion(
                &format!("Directory does not exist: {}", dir),
                &format!("Run 'progflow edit {}' to update the directory path", name),
            ));
        }
    }

    if let Some(ref note) = config.last_note {
        if !quiet {
            println!("Previous note: {}", note);
        }
    }

    if edit_note {
        let editor = get_editor()
            .ok_or_else(|| AppError::User("No editor set. Set $EDITOR or $VISUAL".to_string()))?;

        let temp_file = std::env::temp_dir().join(format!("progflow_note_{}.txt", name));
        if let Some(ref note) = config.last_note {
            std::fs::write(&temp_file, note)
                .map_err(|e| AppError::Io(temp_file.display().to_string(), e))?;
        } else {
            std::fs::write(&temp_file, "")
                .map_err(|e| AppError::Io(temp_file.display().to_string(), e))?;
        }

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", editor, temp_file.display()))
            .status()
            .map_err(|e| AppError::Io("editor".to_string(), e))?;

        if status.success() {
            let new_note = std::fs::read_to_string(&temp_file)
                .map_err(|e| AppError::Io(temp_file.display().to_string(), e))?;
            config.last_note = Some(new_note.trim().to_string());
            save_config(&config)?;
        }
        let _ = std::fs::remove_file(temp_file);
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
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
        }

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

    // Start commands
    for start_cmd in &config.start_commands {
        if verbose {
            eprintln!("Running start command: {}", start_cmd.command);
        }
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&start_cmd.command);

        let cmd_dir = match start_cmd.working_directory.as_deref() {
            Some("home") => dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            Some(dir) if !dir.is_empty() => PathBuf::from(dir),
            _ => PathBuf::from(work_dir),
        };
        cmd.current_dir(cmd_dir);

        if start_cmd.background {
            cmd.stdin(std::process::Stdio::null());
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());

            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                unsafe {
                    cmd.pre_exec(|| {
                        libc::setsid();
                        Ok(())
                    });
                }
            }
        }

        apply_env(&mut cmd, &start_cmd.env);

        match cmd.spawn() {
            Ok(child) => {
                pids.push(child.id());
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to run start command '{}': {}",
                    start_cmd.command, e
                );
            }
        }
    }

    if let Some(ref urls) = config.url_list {
        for url in urls {
            if url.is_empty() {
                continue;
            }
            if !skip_url_check {
                check_url_ready(url);
            }
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
    if !config.start_commands.is_empty() {
        let suffix = if config.start_commands.len() == 1 {
            "command"
        } else {
            "commands"
        };
        parts.push(format!("{} start {}", config.start_commands.len(), suffix));
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

    if !quiet {
        println!("✓ flow '{}' started — {}", name, summary);
    }

    Ok(())
}

fn apply_env(cmd: &mut Command, env_vars: &HashMap<String, String>) {
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
}

fn check_url_ready(url: &str) {
    if !url.contains("localhost") && !url.contains("127.0.0.1") && !url.contains("0.0.0.0") {
        return;
    }

    let host_port = url
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .split('/')
        .next()
        .unwrap_or("");

    if host_port.is_empty() {
        return;
    }

    let addr = if host_port.contains(':') {
        host_port
            .replace("localhost", "127.0.0.1")
            .replace("0.0.0.0", "127.0.0.1")
    } else {
        format!(
            "{}:80",
            host_port
                .replace("localhost", "127.0.0.1")
                .replace("0.0.0.0", "127.0.0.1")
        )
    };

    let ready = if let Ok(mut addrs) = addr.to_socket_addrs() {
        addrs.any(|addr| TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok())
    } else {
        false
    };

    if !ready {
        println!(
            "Warning: {} does not respond – browser may show error",
            host_port
        );
    }
}
