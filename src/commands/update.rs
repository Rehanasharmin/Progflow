use crate::error::AppError;
use std::process::Command;

pub fn run(verbose: bool, quiet: bool) -> Result<(), AppError> {
    let current_version = env!("CARGO_PKG_VERSION");

    if !quiet {
        println!("Checking for updates...");
        if verbose {
            println!("Current version: {}", current_version);
        }
    }

    // Fetch the remote Cargo.toml to check the version
    let remote_toml = Command::new("curl")
        .args(["-sSL", "https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/Cargo.toml"])
        .output()
        .map_err(|e| AppError::Io("fetching remote version".to_string(), e))?;

    if !remote_toml.status.success() {
        return Err(AppError::User("Could not check for updates. Make sure you are connected to the internet.".to_string()));
    }

    let toml_content = String::from_utf8_lossy(&remote_toml.stdout);
    let remote_version = toml_content
        .lines()
        .find(|line| line.starts_with("version = \""))
        .and_then(|line| line.split('"').nth(1))
        .ok_or_else(|| AppError::User("Could not parse remote version information.".to_string()))?;

    if !quiet {
        if verbose {
            println!("Latest version: {}", remote_version);
        }
    }

    if remote_version == current_version {
        if !quiet {
            println!("Progflow is already up to date (v{})!", current_version);
        }
        return Ok(());
    }

    if !quiet {
        println!("A new version is available: v{} (current: v{})", remote_version, current_version);
        println!("Initiating self-update...");
    }

    // Command to fetch and run the install script with 'update' argument
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash -s -- update")
        .status()
        .map_err(|e| AppError::Io("self-update".to_string(), e))?;

    if status.success() {
        if !quiet {
            println!("Progflow has been successfully updated to v{}!", remote_version);
        }
        Ok(())
    } else {
        Err(AppError::User(format!(
            "Self-update failed with exit code: {:?}",
            status.code()
        )))
    }
}
