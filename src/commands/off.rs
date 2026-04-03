use std::io::{self, IsTerminal, Write};
use std::process::Command;

use crate::config::{delete_lock_file, find_active_flow, load_config, read_lock_file, save_config};
use crate::error::AppError;

pub fn run(name: Option<&str>, verbose: bool) -> Result<(), AppError> {
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
        let output = Command::new("kill")
            .arg("-TERM")
            .arg(pid.to_string())
            .output();

        match output {
            Ok(out) if out.status.success() => {}
            Ok(out) => {
                let code = out.status.code().unwrap_or(-1);
                if code != 3 {
                    eprintln!(
                        "Warning: Failed to terminate process {}: exit code {}",
                        pid, code
                    );
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to terminate PID {}: {}", pid, e);
            }
        }
    }

    let is_interactive = io::stdin().is_terminal();

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

    println!("✓ flow '{}' stopped", name);

    Ok(())
}
