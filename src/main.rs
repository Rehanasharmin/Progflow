mod commands;
mod config;
mod error;
mod platform;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

use commands::{delete, edit, list, new, note, off, on, status};
use error::AppError;

#[derive(Parser)]
#[command(name = "progflow")]
#[command(version = "0.1.3")]
#[command(about = "A context-aware workspace manager for Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, global = true, help = "Enable verbose output")]
    verbose: bool,
    #[arg(short, long, global = true, help = "Suppress output")]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Activate a named workspace flow")]
    On { name: String },
    #[command(about = "Deactivate current or named flow")]
    Off {
        name: Option<String>,
        #[arg(short, long, help = "Skip saving note prompt")]
        force: bool,
    },
    #[command(about = "List all configured flows")]
    List {
        #[arg(short, long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Open the config file in $EDITOR or update it via flags")]
    Edit {
        name: String,
        #[arg(long, help = "Set a note for the flow")]
        set_note: Option<String>,
    },
    #[command(about = "Scaffold a new flow config file")]
    New {
        name: String,
        #[arg(long, help = "Working directory")]
        dir: Option<String>,
        #[arg(long, help = "Editor command")]
        editor: Option<String>,
        #[arg(long, help = "Comma-separated URLs")]
        urls: Option<String>,
        #[arg(long, help = "Comma-separated environment variables (KEY=VALUE)")]
        env: Option<String>,
        #[arg(long, help = "Shell path")]
        shell: Option<String>,
    },
    #[command(about = "Print the last saved context note for a flow")]
    Note { name: String },
    #[command(about = "Show status of active flow")]
    Status,
    #[command(about = "Delete a flow")]
    Delete {
        name: String,
        #[arg(short, long, help = "Skip confirmation")]
        force: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::On { name } => on::run(&name, cli.verbose, cli.quiet),
        Commands::Off { name, force } => off::run(name.as_deref(), force, cli.verbose, cli.quiet),
        Commands::List { json } => list::run(json),
        Commands::Edit { name, set_note } => edit::run(&name, set_note, cli.quiet),
        Commands::New {
            name,
            dir,
            editor,
            urls,
            env,
            shell,
        } => new::run(&name, dir, editor, urls, env, shell, cli.quiet),
        Commands::Note { name } => note::run(&name),
        Commands::Status => status::run(cli.verbose),
        Commands::Delete { name, force } => delete::run(&name, force, cli.quiet),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if cli.verbose {
                eprintln!("Error: {:?}", e);
            } else {
                eprintln!("Error: {}", e);
            }
            let code = match e {
                AppError::User(_) => 1,
                AppError::Io(_, _) => 2,
                AppError::Json(_, _) => 2,
                AppError::Config(_) => 1,
            };
            ExitCode::from(code)
        }
    }
}
