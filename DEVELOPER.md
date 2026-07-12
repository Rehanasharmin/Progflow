# Progflow: Developer Documentation and Technical Specification

This document provides a technical overview of the Progflow architecture, internal logic, and operational workflows.

## Architecture Overview

Progflow is a modular CLI utility written in Rust. It focuses on deterministic behavior, minimal resource overhead, and cross-platform compatibility.

### Project Module Map

| Module | Responsibility | Technical Details |
| :--- | :--- | :--- |
| **`main.rs`** | Entry Point | Manages CLI dispatching via `clap`. |
| **`config.rs`** | Data Layer | Handles JSON serialization and flow validation. |
| **`platform.rs`** | OS Abstraction | Normalizes behavior for Linux, macOS, and Termux. |
| **`tips.rs`** | User Intelligence | Provides contextual tips using system entropy for randomness. |
| **`error.rs`** | Fault Management | Implements custom `AppError` for consistent reporting. |
| **`commands/`** | Subcommand Logic | Specific logic for each command (`on`, `off`, `update`, etc.). |

## Process Lifecycle and Security

### Activation Workflow (`progflow on`)

1. **Safety Checks**: Validates flow name to prevent path traversal (blocking `..` and hidden files).
2. **Process Spawning**: Uses `sh -c` with shell parameter passing (e.g., `"$1"`) to prevent command injection from file paths.
3. **Detachment**: On Unix, uses `setsid()` via `pre_exec` to detach child processes.
4. **Liveness**: Uses `libc::kill(pid, 0)` for reliable process checking on Unix and Termux.

### Termination Workflow (`progflow off`)

1. **Graceful Shutdown**: Sends `SIGTERM` to all tracked PIDs.
2. **Forced Cleanup**: Waits 3 seconds and sends `SIGKILL` if processes are still alive.
3. **Lock Management**: Deletes the lock file once all processes are accounted for.

## Update and Installation

### Update Mechanism

The `update` command fetches the remote `Cargo.toml` from the master branch on GitHub. It compares the remote version with the local version and only triggers the installation script if a newer version is available.

### Installation Script (`install.sh`)

- **Platform Detection**: Automatically identifies Linux, macOS, and Termux.
- **Dependency Handling**: Installs Git and Rust if they are missing.
- **Storage Optimization**: Offers an optional step to remove build dependencies (like Rust) after installation to save space.

## Technical Standards

### Security
- **Command Injection**: All external command invocations are hardened against injection.
- **Path Traversal**: Flow names are strictly validated before being used in file operations.
- **Temporary Files**: Uses the `tempfile` crate for secure temporary file handling.

### Platform Support
- **Linux**: Full support with `xdg-open` integration.
- **macOS**: Integration with the native `open` command.
- **Termux**: Support for `pkg` management and `termux-open-url`.

---

Technical Support: [GitHub Issues](https://github.com/Rehanasharmin/Progflow/issues)
License: MIT
