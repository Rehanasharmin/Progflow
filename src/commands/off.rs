use std::io::{self, IsTerminal, Write};
use std::process::Command;

use crate::config::{delete_lock_file, find_active_flow, load_config, read_lock_file, save_config};
use crate::error::AppError;

pub fn run(name: Option<&str>, force: bool, verbose: bool, quiet: bool) -> Result<(), AppError> {
    let name = match name {
        Some(n) => n.to_string(),
        None => match find_active_flow()? {
            Some(n) => n,
            None => return Err(AppError::User("No active flow found".to_string())),
        },
    };

    let lock = match read_lock_file(&name) {
        Ok(l) => l,
        Err(AppError::Io(_, e)) if e.kind() == io::ErrorKind::NotFound => {
            return Err(AppError::User(format!(
                "No lock file found for flow '{}'",
                name
            )));
        }
        Err(e) => return Err(e),
    };

    if verbose {
        eprintln!("Terminating {} processes", lock.pids.len());
    }

    for pid in &lock.pids {
        if verbose {
            eprintln!("Sending SIGTERM to PID {}", pid);
        }
        let _ = Command::new("kill")
            .arg("-TERM")
            .arg(pid.to_string())
            .status();
    }

    // Wait for up to 3 seconds for processes to exit
    let start = std::time::Instant::now();
    let mut pending_pids = lock.pids.clone();

    while !pending_pids.is_empty() && start.elapsed().as_secs() < 3 {
        pending_pids.retain(|pid| {
            // Check if process still exists
            let status = Command::new("kill").arg("-0").arg(pid.to_string()).status();
            status.map(|s| s.success()).unwrap_or(false)
        });
        if !pending_pids.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    for pid in pending_pids {
        if verbose {
            eprintln!("Sending SIGKILL to PID {}", pid);
        }
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .status();
    }

    let is_interactive = io::stdin().is_terminal() && !force;

    if is_interactive {
        print!("Save a context note? [y/N]: ");
        io::stdout()
            .flush()
            .map_err(|e| AppError::Io("stdout".to_string(), e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;

        if input.trim().to_lowercase() == "y" {
            print!("Enter note: ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;

            let mut note = String::new();
            io::stdin()
                .read_line(&mut note)
                .map_err(|e| AppError::Io("stdin".to_string(), e))?;

            let mut config = load_config(&name)?;
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
            config.note = format!("[{}] {}", timestamp, note.trim());
            save_config(&config)?;
        }
    }

    delete_lock_file(&name)?;

    if !quiet {
        println!("✓ flow '{}' stopped", name);
    }

    Ok(())
}
