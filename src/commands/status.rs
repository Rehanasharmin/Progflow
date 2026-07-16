use crate::config::{find_active_flow, load_config, read_lock_file};
use crate::error::AppError;
#[cfg(not(unix))]
use std::process::Command;

#[cfg(unix)]
fn is_process_group_alive(pid: u32) -> bool {
    unsafe {
        let pgid = -(pid as libc::pid_t);
        let res = libc::kill(pgid, 0);
        res == 0 || (std::io::Error::last_os_error().raw_os_error().unwrap_or(0) != libc::ESRCH)
    }
}

#[cfg(not(unix))]
fn is_process_group_alive(pid: u32) -> bool {
    Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn run(json_output: bool, verbose: bool, quiet: bool) -> Result<(), AppError> {
    let active_flow = find_active_flow()?;

    if json_output {
        let json = match active_flow {
            Some(name) => {
                let mut running_count = 0;
                let mut pids = Vec::new();
                if let Ok(lock) = read_lock_file(&name) {
                    pids = lock.pids.clone();
                    for pid in &lock.pids {
                        if is_process_group_alive(*pid) {
                            running_count += 1;
                        }
                    }
                }
                let config = load_config(&name)?;
                serde_json::json!({
                    "active": true,
                    "name": name,
                    "runningProcesses": running_count,
                    "pids": pids,
                    "note": config.note,
                    "lastNote": config.last_note
                })
            }
            None => serde_json::json!({
                "active": false
            }),
        };
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        return Ok(());
    }

    match active_flow {
        Some(name) => {
            if !quiet {
                println!("Active flow: {}", name);
            }

            if let Ok(lock) = read_lock_file(&name) {
                let mut running_count = 0;
                for pid in &lock.pids {
                    if is_process_group_alive(*pid) {
                        running_count += 1;
                    }
                }
                if !quiet {
                    println!("Running processes: {}", running_count);
                } else {
                    println!("{}", running_count);
                }
            }

            let config = load_config(&name)?;
            if !config.note.is_empty() {
                if !quiet {
                    if verbose {
                        println!("Note saved: {}", config.note);
                    } else {
                        println!("{}", config.note);
                    }
                }
            } else if !quiet {
                println!("(no note saved)");
            }
            Ok(())
        }
        None => {
            if !quiet {
                println!("No active flow");
            }
            Ok(())
        }
    }
}
