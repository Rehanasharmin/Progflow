use std::io::{self, IsTerminal, Write};
#[cfg(not(unix))]
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
                "Flow '{}' doesn't seem to be running (no lock file)",
                name
            )));
        }
        Err(e) => return Err(e),
    };

    // Let's see how long this session lasted
    if let Some(start_time_iso) = &lock.start_time {
        if let Ok(start_time) = chrono::DateTime::parse_from_rfc3339(start_time_iso) {
            let now = chrono::Local::now();
            let duration = now.signed_duration_since(start_time);
            let seconds = duration.num_seconds().max(0) as u64;

            if let Ok(mut config) = load_config(&name) {
                config.total_seconds += seconds;
                config.session_count += 1;
                let _ = save_config(&config);
            }
        }
    }

    if verbose {
        eprintln!("Terminating {} processes", lock.pids.len());
    }

    for pid in &lock.pids {
        #[cfg(unix)]
        let alive = unsafe {
            let res = libc::kill(*pid as libc::pid_t, 0);
            res == 0 || (std::io::Error::last_os_error().raw_os_error().unwrap_or(0) != libc::ESRCH)
        };
        #[cfg(not(unix))]
        let alive = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !alive {
            continue;
        }

        if verbose {
            eprintln!("Sending SIGTERM to PID {}", pid);
        }

        #[cfg(unix)]
        unsafe {
            libc::kill(*pid as libc::pid_t, libc::SIGTERM);
        }
        #[cfg(not(unix))]
        {
            let _ = Command::new("kill").arg(pid.to_string()).output();
        }
    }

    // Give the processes a few seconds to shut down nicely
    if !lock.pids.is_empty() {
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    for pid in &lock.pids {
        #[cfg(unix)]
        let alive = unsafe {
            let res = libc::kill(*pid as libc::pid_t, 0);
            res == 0 || (std::io::Error::last_os_error().raw_os_error().unwrap_or(0) != libc::ESRCH)
        };
        #[cfg(not(unix))]
        let alive = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if alive {
            if verbose {
                eprintln!("Sending SIGKILL to PID {}", pid);
            }
            #[cfg(unix)]
            unsafe {
                libc::kill(*pid as libc::pid_t, libc::SIGKILL);
            }
            #[cfg(not(unix))]
            {
                let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
            }
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
        crate::tips::show_tip(crate::tips::TipEvent::Off);
    }

    Ok(())
}
