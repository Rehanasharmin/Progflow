use crate::config::{find_active_flow, load_config, read_lock_file};
use crate::error::AppError;
use std::process::Command;

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
                        let output = Command::new("kill").arg("-0").arg(pid.to_string()).output();
                        if let Ok(out) = output {
                            if out.status.success() {
                                running_count += 1;
                            }
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
                    let output = Command::new("kill").arg("-0").arg(pid.to_string()).output();

                    if let Ok(out) = output {
                        if out.status.success() {
                            running_count += 1;
                        }
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
