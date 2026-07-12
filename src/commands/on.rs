use std::collections::HashMap;
use std::io::{self, IsTerminal, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crate::config::{find_active_flow, load_config, save_config, write_lock_file};
use crate::error::AppError;
use crate::platform::{get_editor, spawn_url};

pub fn run(
    name: &str,
    skip_url_check: bool,
    edit_note: bool,
    note_arg: Option<String>,
    switch: bool,
    verbose: bool,
    quiet: bool,
) -> Result<(), AppError> {
    // If another flow is already running, we should handle that first
    if let Some(active) = find_active_flow()? {
        if active == name {
            if crate::config::is_flow_active(name)? {
                return Err(AppError::User(format!(
                    "Flow '{}' is already active. Stop it first with 'progflow off {}'",
                    name, name
                )));
            }
        } else {
            // A different flow is active
            let proceed = if switch {
                true
            } else if io::stdin().is_terminal() {
                print!(
                    "Flow '{}' is active. Stop it and switch to '{}'? [y/N]: ",
                    active, name
                );
                io::stdout()
                    .flush()
                    .map_err(|e| AppError::Io("stdout".to_string(), e))?;
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .map_err(|e| AppError::Io("stdin".to_string(), e))?;
                answer.trim().to_lowercase() == "y"
            } else {
                false
            };

            if proceed {
                if !quiet {
                    println!("Stopping flow '{}'...", active);
                }
                crate::commands::off::run(Some(&active), true, None, verbose, quiet)?;
            } else {
                return Err(AppError::User(format!(
                    "Another flow ('{}') is currently active. Use --switch to transition automatically.",
                    active
                )));
            }
        }
    }

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

    // Record when we last started this flow
    let now_iso = chrono::Local::now().to_rfc3339();
    config.last_activated = Some(now_iso.clone());

    if let Some(note) = note_arg {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        let formatted_note = format!("[{}] {}", timestamp, note.trim());
        config.note = formatted_note.clone();
        config.last_note = Some(formatted_note);
    }

    if let Some(ref note) = config.last_note {
        if !quiet {
            println!("Previous note: {}", note);
        }
    }

    if edit_note {
        let editor = get_editor()
            .ok_or_else(|| AppError::User("No editor set. Set $EDITOR or $VISUAL".to_string()))?;

        let mut temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| AppError::Io("temporary file".to_string(), e))?;

        if let Some(ref note) = config.last_note {
            use std::io::Write;
            temp_file
                .write_all(note.as_bytes())
                .map_err(|e| AppError::Io("temporary file".to_string(), e))?;
        }

        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} \"$1\"", editor))
            .arg("--")
            .arg(temp_file.path())
            .spawn()
            .map_err(|e| AppError::Io("editor".to_string(), e))?
            .wait()
            .map_err(|e| AppError::Io("editor".to_string(), e))?;

        if status.success() {
            let new_note = std::fs::read_to_string(temp_file.path())
                .map_err(|e| AppError::Io("temporary file".to_string(), e))?;
            config.last_note = Some(new_note.trim().to_string());
            config.note = config.last_note.as_ref().cloned().unwrap_or_default();
        }
    }

    // Save analytics and note updates
    save_config(&config)?;

    let mut pids: Vec<u32> = Vec::new();

    let work_dir = config.directory.as_deref().unwrap_or(".");

    if let Some(ref editor_cmd) = config.editor_cmd {
        if verbose {
            eprintln!("Spawning editor: {}", editor_cmd);
        }
        let mut cmd = Command::new(&config.shell);
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
                if verbose {
                    eprintln!("Editor spawned with PID {}", child.id());
                }

                // Wait a tiny bit to see if the editor crashes right away
                std::thread::sleep(std::time::Duration::from_millis(200));
                #[cfg(unix)]
                let alive = unsafe { libc::kill(child.id() as libc::pid_t, 0) == 0 };
                #[cfg(not(unix))]
                let alive = Command::new("kill")
                    .arg("-0")
                    .arg(child.id().to_string())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                if !alive && !quiet {
                    println!(
                        "Warning: Editor '{}' (PID {}) exited immediately.",
                        editor_cmd,
                        child.id()
                    );
                }
            }
            Err(e) => {
                return Err(AppError::User(format!(
                    "Failed to spawn editor '{}': {}. Check if the command exists and is in your PATH.",
                    editor_cmd, e
                )));
            }
        }
    }

    // Time to run any background commands the user set up
    let log_path = crate::config::get_log_path(name)?;
    let log_dir = crate::config::get_log_dir()?;
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| AppError::Io(log_dir.display().to_string(), e))?;

    // Clear log file on start
    let _ = std::fs::File::create(&log_path);

    for start_cmd in &config.start_commands {
        if verbose {
            eprintln!("Running start command: {}", start_cmd.command);
        }
        let mut cmd = Command::new(&config.shell);
        cmd.arg("-c").arg(&start_cmd.command);

        let cmd_dir = match start_cmd.working_directory.as_deref() {
            Some("home") => dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            Some(dir) if !dir.is_empty() => PathBuf::from(dir),
            _ => PathBuf::from(work_dir),
        };
        cmd.current_dir(cmd_dir);

        if start_cmd.background {
            cmd.stdin(std::process::Stdio::null());

            // Redirect output to log file
            let log_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .map_err(|e| AppError::Io(log_path.display().to_string(), e))?;
            let log_file_err = log_file
                .try_clone()
                .map_err(|e| AppError::Io(log_path.display().to_string(), e))?;

            cmd.stdout(log_file);
            cmd.stderr(log_file_err);

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
                if verbose {
                    eprintln!("Start command spawned with PID {}", child.id());
                }

                // Small delay to check for immediate failure
                std::thread::sleep(std::time::Duration::from_millis(200));
                #[cfg(unix)]
                let alive = unsafe { libc::kill(child.id() as libc::pid_t, 0) == 0 };
                #[cfg(not(unix))]
                let alive = Command::new("kill")
                    .arg("-0")
                    .arg(child.id().to_string())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);
                if !alive && !quiet {
                    println!(
                        "Warning: Start command '{}' (PID {}) exited immediately.",
                        start_cmd.command,
                        child.id()
                    );
                }
            }
            Err(e) => {
                return Err(AppError::User(format!(
                    "Failed to run start command '{}': {}. Check if the command exists and is in your PATH.",
                    start_cmd.command, e
                )));
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

    // Create a lock file so we know this flow is running
    write_lock_file(name, pids.clone(), Some(now_iso))?;

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
        println!("✓ flow '{}' started: {}", name, summary);
        crate::tips::show_tip(crate::tips::TipEvent::On);
    }

    Ok(())
}

fn apply_env(cmd: &mut Command, env_vars: &HashMap<String, String>) {
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
}

fn check_url_ready(url: &str) {
    let url_lower = url.to_lowercase();
    if !url_lower.contains("localhost")
        && !url_lower.contains("127.0.0.1")
        && !url_lower.contains("0.0.0.0")
        && !url_lower.contains("[::1]")
    {
        return;
    }

    let host_port = url
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("HTTP://")
        .trim_start_matches("HTTPS://")
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

    // Try a few times with short timeout
    let mut ready = false;
    for _ in 0..5 {
        if let Ok(mut addrs) = addr.to_socket_addrs() {
            if addrs.any(|addr| TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok()) {
                ready = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    if !ready {
        println!(
            "Warning: {} does not respond – browser may show error",
            host_port
        );
    }
}
