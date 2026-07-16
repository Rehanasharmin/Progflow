# Introduction of Progflow

Progflow is a powerful command-line interface utility built in rust, designed to streamline your development workflow. It helps you manage complex project environments, what we call "flows", by bundling your editor configurations, background services, and web resources into a single, easy-to-manage unit. By automating the setup and teardown of these environments, this context-aware workspace manager reduces the mental overhead of switching between projects, allowing you to focus on writing code.

[Official Website](https://progflowcli.netlify.app)

[![rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Termux-lightgrey.svg?style=flat-square)](https://github.com/Rehanasharmin/Progflow)

## Key Features

*   **Project Orchestration**: Launch your editor, start local development servers, and open documentation with one command.
*   **Context Preservation**: Automatically save and restore project-specific notes so you never lose your place.
*   **Smart Service Management**: Run multiple background processes in parallel with automatic lifecycle management and logging.
*   **Service Readiness Detection**: Progflow checks if your local servers are actually listening before attempting to open them in your browser.
*   **Cross-Platform Support**: Optimized for Linux, macOS, and Termux (Android) environments.
*   **Advanced Automation**: Full CLI support for non-interactive scripts and CI-CD pipelines.

## Getting Started

### Installation

The recommended way to install Progflow on Linux or macOS is via our automated script:

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash
```

Alternatively, you can build from source:

```bash
git clone https://github.com/Rehanasharmin/Progflow.git
cd Progflow
cargo build --release
cp target/release/progflow ~/.local/bin/
```

## Documentation

For more detailed information on how to use and contribute to Progflow, please refer to the following documents:

*   [Command Reference](COMMANDS.md): A complete guide to all Progflow commands and their flags.
*   [Installation Safety](INSTALL_SAFETY.md): An in-depth look at the security and operations of our installation script.
*   [Developer Guide](DEVELOPER.md): Technical details on the architecture and internal logic for those who want to contribute.
*   [Contributing](CONTRIBUTORS.md): Guidelines on how to get involved and help improve the project.

## Usage Guide

### Activating a Flow

To start a workspace:
```bash
progflow on <name>
```
Flags:
*   `--skip-url-check`: Skip waiting for local servers to be ready.
*   `--edit-note`: Open your default editor to update the project note before starting.
*   `--note "..."`: Directly set a note via the command line.
*   `--switch`: Automatically stop any active flow before starting this one.

### Stopping a Flow

To gracefully shut down your environment:
```bash
progflow off [name]
```
If no name is provided, the currently active flow is stopped.

### Managing Flows

*   **`progflow list`**: View all your configured flows.
*   **`progflow status`**: See the current state of an active flow, including running processes.
*   **`progflow logs <name>`**: View the output streams of your background services.
*   **`progflow stats <name>`**: Check your productivity stats, like total time spent in a flow.

### Creating and Editing

*   **`progflow new <name>`**: Scaffold a new project environment. This command is interactive by default but supports numerous flags for automation.
*   **`progflow edit <name>`**: Modify an existing configuration.

### Shell Integration

For maximum speed, you can generate shell aliases for all your flows:
```bash
eval "$(progflow aliases)"
```
Add this to your .bashrc or .zshrc to activate projects using flow-<name>.

## Technical Details

Progflow is designed for reliability and safety:

*   **Conflict Prevention**: Uses a locking mechanism to prevent multiple instances of the same flow from running simultaneously.
*   **Graceful Termination**: Background processes are managed through a tiered sequence, starting with SIGTERM and escalating to SIGKILL only if necessary.
*   **Privacy and Portability**: Configurations are stored as simple JSON files in ~/.config/flow/, making them easy to back up or sync.

## License

Progflow is open-source software licensed under the MIT License.
