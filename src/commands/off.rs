use std::io::{self, IsTerminal, Write};

use crate::config::{delete_lock_file, find_active_flow, load_config, read_lock_file, save_config};
use crate::error::AppError;

pub fn run(name: Option<&str>) -> Result<(), AppError> {
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

    for pid in &lock.pids {
        unsafe {
            let result = libc::kill(*pid as libc::pid_t, libc::SIGTERM);
            if result != 0 && io::Error::last_os_error().raw_os_error() != Some(3) {
                eprintln!(
                    "Warning: Failed to send SIGTERM to PID {}: {}",
                    pid,
                    io::Error::last_os_error()
                );
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
            config.note = note.trim().to_string();
            save_config(&config)?;
        }
    }

    delete_lock_file(&name)?;

    println!("✓ flow '{}' stopped", name);

    Ok(())
}
