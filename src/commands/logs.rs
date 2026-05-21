use crate::config::get_log_path;
use crate::error::AppError;
use std::fs;

pub fn run(name: &str, _verbose: bool, _quiet: bool) -> Result<(), AppError> {
    let log_path = get_log_path(name)?;

    if !log_path.exists() {
        println!("(no logs found for flow '{}')", name);
        return Ok(());
    }

    let content = fs::read_to_string(&log_path)
        .map_err(|e| AppError::Io(log_path.display().to_string(), e))?;

    if content.is_empty() {
        println!("(logs are empty)");
    } else {
        println!("{}", content);
    }

    Ok(())
}
