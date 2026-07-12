use std::collections::HashMap;
use std::io::{self, IsTerminal, Write};

use crate::config::{save_config, FlowConfig, StartCommand};
use crate::error::AppError;

#[allow(clippy::too_many_arguments)]
pub fn run(
    name: &str,
    dir: Option<String>,
    editor: Option<String>,
    urls: Option<String>,
    env: Option<String>,
    shell: Option<String>,
    start_commands_json: Option<String>,
    cmds: Vec<String>,
    cmd_dirs: Vec<String>,
    cmd_bgs: Vec<String>,
    quiet: bool,
) -> Result<(), AppError> {
    let is_one_liner = dir.is_some()
        || editor.is_some()
        || urls.is_some()
        || env.is_some()
        || shell.is_some()
        || start_commands_json.is_some()
        || !cmds.is_empty();

    let mut config = FlowConfig {
        name: name.to_string(),
        directory: dir,
        editor_cmd: editor,
        url_list: urls.map(|u| u.split(',').map(|s| s.trim().to_string()).collect()),
        shell: shell.unwrap_or_else(|| "/bin/sh".to_string()),
        env: HashMap::new(),
        note: String::new(),
        start_commands: Vec::new(),
        last_note: None,
        total_seconds: 0,
        session_count: 0,
        last_activated: None,
    };

    if let Some(env_str) = env {
        for pair in env_str.split(',') {
            if let Some((key, value)) = pair.split_once('=') {
                config
                    .env
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    if let Some(json) = start_commands_json {
        config.start_commands = serde_json::from_str(&json)
            .map_err(|e| AppError::User(format!("Invalid start commands JSON: {}", e)))?;
    } else if !cmds.is_empty() {
        for (i, cmd) in cmds.into_iter().enumerate() {
            let working_directory = cmd_dirs.get(i).cloned();
            let background = cmd_bgs.get(i).map(|s| s == "true").unwrap_or(true);
            config.start_commands.push(StartCommand {
                command: cmd,
                working_directory,
                env: HashMap::new(),
                background,
            });
        }
    }

    if !is_one_liner {
        // Interactive mode (or piped input)
        if io::stdin().is_terminal() && !quiet {
            println!("Creating new flow: {}", name);
        }

        let mut input_buf = String::new();

        if io::stdin().is_terminal() && !quiet {
            print!("Enter working directory [.] (or 'home'): ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        }
        input_buf.clear();
        io::stdin()
            .read_line(&mut input_buf)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        let input = input_buf.trim();
        if !input.is_empty() {
            config.directory = Some(input.to_string());
        }

        if io::stdin().is_terminal() && !quiet {
            print!("Enter command to open your editor (e.g. 'code .'): ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        }
        input_buf.clear();
        io::stdin()
            .read_line(&mut input_buf)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        let input = input_buf.trim();
        if !input.is_empty() {
            config.editor_cmd = Some(input.to_string());
        }

        if io::stdin().is_terminal() && !quiet {
            print!("Enter URLs (comma-separated): ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        }
        input_buf.clear();
        io::stdin()
            .read_line(&mut input_buf)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        let input = input_buf.trim();
        if !input.is_empty() {
            config.url_list = Some(
                input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            );
        }

        if io::stdin().is_terminal() && !quiet {
            print!("Enter shell (e.g., /bin/bash) [default /bin/sh]: ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        }
        input_buf.clear();
        io::stdin()
            .read_line(&mut input_buf)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        let input = input_buf.trim();
        if !input.is_empty() {
            config.shell = input.to_string();
        }

        if io::stdin().is_terminal() && !quiet {
            print!("Enter environment variables (KEY=value, comma-separated): ");
            io::stdout()
                .flush()
                .map_err(|e| AppError::Io("stdout".to_string(), e))?;
        }
        input_buf.clear();
        io::stdin()
            .read_line(&mut input_buf)
            .map_err(|e| AppError::Io("stdin".to_string(), e))?;
        let input = input_buf.trim();
        if !input.is_empty() {
            for pair in input.split(',') {
                if let Some((key, value)) = pair.split_once('=') {
                    config
                        .env
                        .insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        loop {
            if io::stdin().is_terminal() && !quiet {
                print!("Add a start command (y/n): ");
                io::stdout()
                    .flush()
                    .map_err(|e| AppError::Io("stdout".to_string(), e))?;
            }
            input_buf.clear();
            if io::stdin().read_line(&mut input_buf).is_err() || input_buf.trim().is_empty() {
                break;
            }
            if input_buf.trim().to_lowercase() != "y" {
                break;
            }

            if io::stdin().is_terminal() && !quiet {
                print!("  Command: ");
                io::stdout()
                    .flush()
                    .map_err(|e| AppError::Io("stdout".to_string(), e))?;
            }
            input_buf.clear();
            io::stdin()
                .read_line(&mut input_buf)
                .map_err(|e| AppError::Io("stdin".to_string(), e))?;
            let command = input_buf.trim().to_string();

            if io::stdin().is_terminal() && !quiet {
                print!("  Working directory (enter for flow dir, or 'home'): ");
                io::stdout()
                    .flush()
                    .map_err(|e| AppError::Io("stdout".to_string(), e))?;
            }
            input_buf.clear();
            io::stdin()
                .read_line(&mut input_buf)
                .map_err(|e| AppError::Io("stdin".to_string(), e))?;
            let working_directory = if input_buf.trim().is_empty() {
                None
            } else {
                Some(input_buf.trim().to_string())
            };

            if io::stdin().is_terminal() && !quiet {
                print!("  Run in background (y/n) [y]: ");
                io::stdout()
                    .flush()
                    .map_err(|e| AppError::Io("stdout".to_string(), e))?;
            }
            input_buf.clear();
            io::stdin()
                .read_line(&mut input_buf)
                .map_err(|e| AppError::Io("stdin".to_string(), e))?;
            let background = input_buf.trim().to_lowercase() != "n";

            config.start_commands.push(StartCommand {
                command,
                working_directory,
                env: HashMap::new(),
                background,
            });
        }
    }

    config.validate()?;
    save_config(&config)?;

    if !quiet {
        println!("✓ flow '{}' created", name);
        crate::tips::show_tip(crate::tips::TipEvent::Create);
    }

    Ok(())
}
