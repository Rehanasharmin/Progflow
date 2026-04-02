mod commands;
mod config;
mod error;
mod platform;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

use commands::{edit, list, new, note, off, on};
use error::AppError;

#[derive(Parser)]
#[command(name = "progflow")]
#[command(version = "0.1.0")]
#[command(about = "A context-aware workspace manager for Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Activate a named workspace flow")]
    On { name: String },
    #[command(about = "Deactivate current or named flow")]
    Off { name: Option<String> },
    #[command(about = "List all configured flows")]
    List,
    #[command(about = "Open the config file in $EDITOR")]
    Edit { name: String },
    #[command(about = "Scaffold a new flow config file interactively")]
    New { name: String },
    #[command(about = "Print the last saved context note for a flow")]
    Note { name: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::On { name } => on::run(&name),
        Commands::Off { name } => off::run(name.as_deref()),
        Commands::List => list::run(),
        Commands::Edit { name } => edit::run(&name),
        Commands::New { name } => new::run(&name),
        Commands::Note { name } => note::run(&name),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let code = match e {
                AppError::User(_) => 1,
                AppError::Io(_, _) => 2,
                AppError::Json(_, _) => 2,
            };
            eprintln!("Error: {}", e);
            ExitCode::from(code)
        }
    }
}
