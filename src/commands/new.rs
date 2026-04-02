use std::collections::HashMap;
use std::io::{self, IsTerminal, Write};

use crate::config::{get_config_path, save_config, FlowConfig};
use crate::error::AppError;

pub fn run(name: &str) -> Result<(), AppError> {
    let config_path = get_config_path(name)?;

    if config_path.exists() {
        return Err(AppError::User(format!("Flow '{}' already exists", name)));
    }

    let is_interactive = io::stdin().is_terminal();

    if is_interactive {
        run_interactive(name)
    } else {
        run_non_interactive(name)
    }
}

fn run_interactive(name: &str) -> Result<(), AppError> {
    println!("Creating new flow '{}'", name);

    print!("Working directory (optional, press Enter to skip): ");
    io::stdout()
        .flush()
        .map_err(|e| AppError::Io("stdout".to_string(), e))?;
    let mut directory = String::new();
    io::stdin()
        .read_line(&mut directory)
        .map_err(|e| AppError::Io("stdin".to_string(), e))?;
    let directory: Option<String> = if directory.trim().is_empty() {
        None
    } else {
        Some(directory.trim().to_string())
    };

    print!("Editor command (optional, e.g. 'vim .'): ");
    io::stdout()
        .flush()
        .map_err(|e| AppError::Io("stdout".to_string(), e))?;
    let mut editor_cmd = String::new();
    io::stdin()
        .read_line(&mut editor_cmd)
        .map_err(|e| AppError::Io("stdin".to_string(), e))?;
    let editor_cmd: Option<String> = if editor_cmd.trim().is_empty() {
        None
    } else {
        Some(editor_cmd.trim().to_string())
    };

    print!("URLs to open (comma-separated, optional): ");
    io::stdout()
        .flush()
        .map_err(|e| AppError::Io("stdout".to_string(), e))?;
    let mut urls_input = String::new();
    io::stdin()
        .read_line(&mut urls_input)
        .map_err(|e| AppError::Io("stdin".to_string(), e))?;
    let url_list: Option<Vec<String>> = if urls_input.trim().is_empty() {
        None
    } else {
        Some(
            urls_input
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        )
    };

    print!("Shell (default: /bin/sh): ");
    io::stdout()
        .flush()
        .map_err(|e| AppError::Io("stdout".to_string(), e))?;
    let mut shell = String::new();
    io::stdin()
        .read_line(&mut shell)
        .map_err(|e| AppError::Io("stdin".to_string(), e))?;
    let shell: String = if shell.trim().is_empty() {
        "/bin/sh".to_string()
    } else {
        shell.trim().to_string()
    };

    print!("Environment variables (KEY=VALUE, comma-separated, optional): ");
    io::stdout()
        .flush()
        .map_err(|e| AppError::Io("stdout".to_string(), e))?;
    let mut env_input = String::new();
    io::stdin()
        .read_line(&mut env_input)
        .map_err(|e| AppError::Io("stdin".to_string(), e))?;
    let env: HashMap<String, String> = if env_input.trim().is_empty() {
        HashMap::new()
    } else {
        env_input
            .trim()
            .split(',')
            .filter_map(|s| {
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    };

    let config = FlowConfig {
        name: name.to_string(),
        directory,
        editor_cmd,
        url_list,
        shell,
        env,
        note: String::new(),
    };

    save_config(&config)?;

    println!("✓ flow '{}' created", name);

    Ok(())
}

fn run_non_interactive(name: &str) -> Result<(), AppError> {
    let config = FlowConfig {
        name: name.to_string(),
        directory: None,
        editor_cmd: None,
        url_list: None,
        shell: "/bin/sh".to_string(),
        env: HashMap::new(),
        note: String::new(),
    };

    save_config(&config)?;

    println!("✓ flow '{}' created (non-interactive mode)", name);
    println!("  Use 'progflow edit {}' to configure", name);

    Ok(())
}
