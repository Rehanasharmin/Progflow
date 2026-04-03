use crate::config::{find_active_flow, load_config};
use crate::error::AppError;

pub fn run(verbose: bool) -> Result<(), AppError> {
    let active_flow = find_active_flow()?;

    match active_flow {
        Some(name) => {
            println!("Active flow: {}", name);
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
