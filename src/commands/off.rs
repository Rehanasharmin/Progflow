use std::io::{self, IsTerminal, Write};
use std::process::Command;

use crate::config::{delete_lock_file, find_active_flow, load_config, read_lock_file, save_config};
use crate::error::AppError;

pub fn run(
    name: Option<&str>,
    force: bool,
    note_arg: Option<String>,
    verbose: bool,
    quiet: bool,
) -> Result<(), AppError> {
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
            .arg(pid.to_string())
            .output()
            .map_err(|e| AppError::Io("kill".to_string(), e))?;

        if !output.status.success() && verbose {
            let err = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "Warning: Failed to terminate process {}: {}",
                pid,
                err.trim()
            );
        }
    }

    // Wait 3 seconds
    if !lock.pids.is_empty() {
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    for pid in &lock.pids {
        let output = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()
            .map_err(|e| AppError::Io("kill".to_string(), e))?;

        if output.status.success() {
            if verbose {
                eprintln!("Sending SIGKILL to PID {}", pid);
            }
            let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
        }
    }

    if let Some(note) = note_arg {
        let mut config = load_config(&name)?;
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        let formatted_note = format!("[{}] {}", timestamp, note.trim());
        config.note = formatted_note.clone();
        config.last_note = Some(formatted_note);
        save_config(&config)?;
    } else if !force && io::stdin().is_terminal() {
        print!("Save a context note? [y/N]: ");
        io::stdout()
            .flush()
            .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        if answer.trim().to_lowercase() == "y" {
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
            let formatted_note = format!("[{}] {}", timestamp, note.trim());
            config.note = formatted_note.clone();
            config.last_note = Some(formatted_note);
            save_config(&config)?;
        }
    }

    delete_lock_file(&name)?;

    if !quiet {
        println!("✓ flow '{}' stopped", name);
    }

    Ok(())
}
