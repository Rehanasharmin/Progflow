use std::process::Command;
use crate::error::AppError;

pub fn run(_verbose: bool, quiet: bool) -> Result<(), AppError> {
    if !quiet {
        println!("🚀 Initiating self-update for Progflow...");
        println!("📦 Fetching the latest version from GitHub...");
    }

    // Command to fetch and run the install script with 'update' argument
    // Using bash -s to pass arguments to the script
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash -s -- update")
        .status()
        .map_err(|e| AppError::Io("self-update".to_string(), e))?;

    if status.success() {
        if !quiet {
            println!("✅ Progflow has been successfully updated!");
        }
        Ok(())
    } else {
        Err(AppError::User(format!(
            "Self-update failed with exit code: {:?}",
            status.code()
        )))
    }
}
