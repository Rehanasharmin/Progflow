## Progflow: Context-Aware Workspace Orchestration Utility

[Official Website](https://progflowcli.netlify.app)

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Termux-lightgrey.svg?style=flat-square)](https://github.com/Rehanasharmin/Progflow)

## Executive Summary

Progflow is a specialized command-line interface (CLI) utility developed in Rust for the orchestration of development environments. It facilitates rapid context switching by encapsulating editor configurations, background services, and web-based resources into unified "flows." By automating the initialization and termination of these environments, Progflow minimizes cognitive load and operational overhead during the development lifecycle.

## Primary Capabilities

- **Environment Orchestration**: Atomic initialization of integrated development environments (IDEs), local service dependencies, and relevant documentation.
- **State Persistence**: Automated capture and restoration of context-aware notes during session transitions.
- **Concurrent Process Management**: Parallel execution of background services with integrated PID tracking and lifecycle management.
- **Network Readiness Validation**: Synchronous TCP connectivity verification for local services prior to browser invocation.
- **Platform Agnostic Intelligence**: Native support and optimized heuristics for Linux, macOS, and Termux environments.
- **Integrated Logging Architecture**: Centralized logging system for monitoring background process telemetry.
- **Programmable Interface**: Comprehensive CLI argument support for non-interactive automation and integration with CI/CD pipelines.

## Operational Reference

### Installation

Automatic installation script for linux and MacOS (Recommended):

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash
```

Manual build from source:

```bash
git clone https://github.com/Rehanasharmin/Progflow.git
cd Progflow
cargo build --release
cp target/release/progflow ~/.local/bin/
```

### Command Reference

#### Global Options
- `-v, --verbose`: Enable detailed operational telemetry.
- `-q, --quiet`: Suppress standard output for concise execution.

#### `progflow on <name>`
Activate a workspace flow and associated processes.
- `--skip-url-check`: Bypass synchronous TCP connectivity verification for local resources.
- `--edit-note`: Invoke the system editor to modify the context note prior to activation.
- `--note <text>`: Set a transient context note for the current session via CLI.

#### `progflow off [name]`
Execute graceful termination of tracked processes.
- `--force`: Bypass the interactive context note prompt.
- `--note <text>`: Persist a final context note to the configuration file.

#### `progflow status`
Retrieve the operational status of the active flow.
- `--json`: Output status metadata in structured JSON format for programmatic ingestion.

#### `progflow list`
Enumerate all configured flows with liveness indicators.
- `--json`: Output flow inventory in structured JSON format.

#### `progflow new <name>`
Initialize a new workspace flow configuration.
- `--dir <path>`: Define the target working directory.
- `--editor <cmd>`: Define the IDE invocation command.
- `--urls <list>`: Comma-separated list of resources for browser invocation.
- `--env <list>`: Comma-separated environment variables in `KEY=VALUE` format.
- `--shell <path>`: Path to the preferred shell interpreter.
- `--start-commands <json>`: Define background services via a JSON array.
- `--cmd <cmd>`: Repeatable flag to define an additional start command.
- `--cmd-dir <path>`: Repeatable flag to define the working directory for the corresponding `--cmd`.
- `--cmd-bg <bool>`: Repeatable flag to define the detachment state for the corresponding `--cmd`.

#### `progflow edit <name>`
Modify an existing flow configuration.
- Supports all flags available in `progflow new` for atomic updates to specific fields.
- `--set-note <text>`: Programmatically update the persistent context note.

#### `progflow note <name>`
Inspect the last persisted context note for the specified flow.

#### `progflow logs <name>`
Inspect standard output and error streams of background services associated with the flow.

#### `progflow stats <name>`
Retrieve usage analytics, including total development time and session frequency.

#### `progflow aliases`
Generate POSIX-compliant shell alias definitions for all configured flows. This allows for instant project activation without typing the full command.

**Automation**: To automatically load these aliases in every terminal session, add the following line to your shell profile (`.bashrc`, `.zshrc`, or `.profile`):
```bash
eval "$(progflow aliases)"
```
Once added, you can simply type `flow-<name>` (e.g., `flow-webapp`) to activate a project.

#### `progflow update`
Automatically update Progflow to the latest version from the source repository.
```bash
progflow update
```

#### `progflow delete <name>` (Alias: `remove`)
Remove a flow configuration and its associated state.
- `--force`: Suppress deletion confirmation prompt.

## Technical Logic and Implementation

### Double Activation Prevention
The utility implements a robust locking mechanism to prevent concurrent activation of the same flow, mitigating risk of process orphaning or state corruption.

### Lifecycle Management
Upon deactivation, Progflow executes a tiered termination sequence, delivering SIGTERM followed by a mandatory wait period and a fallback SIGKILL to ensure complete resource reclamation.

### Non-Interactive Execution
The tool is architected to support piped input and explicit flag-based configuration, enabling seamless integration into automated workflows without requiring a terminal emulator (TTY).

## Configuration Specification

Flow configurations are persisted as JSON objects within the `~/.config/flow/` directory.

```json
{
  "name": "enterprise-service",
  "directory": "/opt/dev/enterprise-service",
  "editorCmd": "nvim .",
  "urlList": ["https://internal-docs.local", "http://localhost:8080"],
  "shell": "/bin/zsh",
  "env": { "NODE_ENV": "production" },
  "startCommands": [
    { "command": "docker-compose up -d", "background": true },
    { "command": "cargo watch -x run", "background": true }
  ],
  "lastNote": "[2024-05-21] Verified auth middleware implementation."
}
```

## Platform Compatibility

- **Linux**: Primary target platform utilizing `xdg-open` for resource invocation.
- **macOS**: Native integration with the `open` utility and BSD-compliant process signals.
- **Termux**: Mobile-optimized deployment supporting `termux-open-url` and `termux-boot` interfaces.

## Contributing

Technical contributions are welcomed. Developers are encouraged to follow established Rust idioms and ensure all integration tests pass prior to pull request submission.

## License

Progflow is distributed under the MIT License. Refer to the `LICENSE` file for full legal text.

---

Designed for professional developers requiring rigorous workspace management.
[Official Website](https://progflowcli.netlify.app) | [Issue Tracking](https://github.com/Rehanasharmin/Progflow/issues)
