use crate::config::{find_active_flow, load_config, read_lock_file};
use crate::error::AppError;
use std::process::Command;

pub fn run(verbose: bool) -> Result<(), AppError> {
    let active_flow = find_active_flow()?;

    match active_flow {
        Some(name) => {
            println!("Active flow: {}", name);

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
                println!("Running processes: {}", running_count);
            }

            let config = load_config(&name)?;
            if !config.note.is_empty() {
                if verbose {
                    println!("Note saved: {}", config.note);
                } else {
                    println!("{}", config.note);
                }
            } else {
                println!("(no note saved)");
            }
            Ok(())
        }
        None => {
            println!("No active flow");
            Ok(())
        }
    }
}
