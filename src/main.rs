mod commands;
mod config;
mod error;
mod platform;
mod tips;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

use commands::{aliases, delete, edit, list, logs, matrix, new, note, off, on, status, update};
use error::AppError;

#[derive(Parser)]
#[command(name = "progflow")]
#[command(version = "0.1.4")]
#[command(about = "A simple way to manage your project workspaces", long_about = None)]
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
    On {
        name: String,
        #[arg(long, help = "Skip URL readiness check")]
        skip_url_check: bool,
        #[arg(long, help = "Edit context note")]
        edit_note: bool,
        #[arg(long, help = "Set a context note")]
        note: Option<String>,
        #[arg(short, long, help = "Auto-switch if another flow is active")]
        switch: bool,
    },
    #[command(about = "Deactivate current or named flow")]
    Off {
        name: Option<String>,
        #[arg(short, long, help = "Skip saving note prompt")]
        force: bool,
        #[arg(long, help = "Save a context note")]
        note: Option<String>,
    },
    #[command(about = "List all configured flows")]
    List {
        #[arg(short, long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Open the config file in $EDITOR or update it via flags")]
    Edit {
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
        #[arg(long, help = "Set a note for the flow")]
        set_note: Option<String>,
        #[arg(long, help = "Start commands as JSON string")]
        start_commands: Option<String>,
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
        #[arg(long, help = "Start commands as JSON string")]
        start_commands: Option<String>,
        #[arg(long = "cmd", help = "Additional start command")]
        cmds: Vec<String>,
        #[arg(long = "cmd-dir", help = "Working directory for start command")]
        cmd_dirs: Vec<String>,
        #[arg(long = "cmd-bg", action = clap::ArgAction::Append, help = "Run start command in background (true/false)")]
        cmd_bgs: Vec<String>,
    },
    #[command(about = "Print the last saved context note for a flow")]
    Note { name: String },
    #[command(about = "Show status of active flow")]
    Status {
        #[arg(short, long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Delete a flow", alias = "remove")]
    Delete {
        name: String,
        #[arg(short, long, help = "Skip confirmation")]
        force: bool,
    },
    #[command(about = "Show logs of start commands for a flow")]
    Logs { name: String },
    #[command(about = "Update progflow to the latest version")]
    Update,
    #[command(about = "Show analytics for a flow")]
    Stats { name: String },
    #[command(about = "Generate shell aliases for all flows")]
    Aliases,
    #[command(hide = true)]
    Matrix,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::On {
            name,
            skip_url_check,
            edit_note,
            note,
            switch,
        } => on::run(
            &name,
            skip_url_check,
            edit_note,
            note,
            switch,
            cli.verbose,
            cli.quiet,
        ),
        Commands::Off { name, force, note } => {
            off::run(name.as_deref(), force, note, cli.verbose, cli.quiet)
        }
        Commands::List { json } => list::run(json, cli.verbose, cli.quiet),
        Commands::Edit {
            name,
            dir,
            editor,
            urls,
            env,
            shell,
            set_note,
            start_commands,
        } => edit::run(
            &name,
            dir,
            editor,
            urls,
            env,
            shell,
            set_note,
            start_commands,
            cli.quiet,
        ),
        Commands::New {
            name,
            dir,
            editor,
            urls,
            env,
            shell,
            start_commands,
            cmds,
            cmd_dirs,
            cmd_bgs,
        } => new::run(
            &name,
            dir,
            editor,
            urls,
            env,
            shell,
            start_commands,
            cmds,
            cmd_dirs,
            cmd_bgs,
            cli.quiet,
        ),
        Commands::Note { name } => note::run(&name, cli.verbose, cli.quiet),
        Commands::Status { json } => status::run(json, cli.verbose, cli.quiet),
        Commands::Delete { name, force } => delete::run(&name, force, cli.verbose, cli.quiet),
        Commands::Logs { name } => logs::run(&name, cli.verbose, cli.quiet),
        Commands::Update => update::run(cli.verbose, cli.quiet),
        Commands::Stats { name } => commands::stats::run(&name),
        Commands::Aliases => aliases::run(),
        Commands::Matrix => matrix::run(),
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
