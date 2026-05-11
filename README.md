# Progflow - Context-Aware Workspace Manager

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow)](https://opensource.org/licenses/MIT)

🌐 Website: [progflowcli.netlify.app](https://progflowcli.netlify.app)

A powerful **CLI tool** for managing development workflows on Linux and Termux. Organize your projects, launch your editor and browsers instantly, and never lose track of your context.

**Use cases:** Workspace management, project switching, development environment automation, context notes, productivity tool for developers.

## What is it?

progflow is a powerful CLI tool that streamlines your development workflow by managing workspace configurations called "flows". Each flow encapsulates your project environment—launching your preferred editor, opening relevant URLs, and setting custom environment variables—everything you need to start working instantly.

When you stop a flow, progflow automatically prompts you to save a context note, preserving your progress and thoughts for your next session. This makes it effortless to switch between projects without losing track of where you left off.

Whether you're managing multiple projects, diving into documentation, or tracking your debugging sessions, progflow keeps everything organized and accessible with a single command.

## Installation

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash
```

Or build from source:
```bash
git clone https://github.com/Rehanasharmin/Progflow.git
cd Progflow
cargo build --release
cp target/release/progflow ~/.local/bin/
```

## Quick Start

```bash
# Interactive creation
progflow new dev

# Non-interactive / Scripted creation (New in 0.1.3!)
progflow new dev --dir /home/you/projects/myapp --editor "code ." --urls "https://github.com" --quiet

progflow on dev      # Start working
progflow off dev    # Stop and save note (auto-skips note prompt if non-interactive)
progflow list       # See all flows
progflow note dev   # Read last note
```

## Use Cases

### 1. Development Environment
Create a flow for each project:
```bash
progflow new backend
# directory: /home/user/projects/api
# editor: code .
# URLs: http://localhost:3000, http://localhost:5432

progflow on backend  # Opens editor, browser, docs all at once
```

### 2. CI/CD or AI Agent Integration
Automate workspace setup without prompts:
```bash
progflow new auto-flow --dir $(pwd) --editor "vim ." --quiet
progflow on auto-flow --quiet
# ... do work ...
progflow edit auto-flow --set-note "Task completed by agent" --quiet
progflow off auto-flow --force --quiet
```

## Commands

Progflow provides commands to manage your workspace flows:

| Command | Description |
|---------|-------------|
| `progflow on <name>` | Activate a workspace flow |
| `progflow off [name]` | Deactivate the current or specified flow |
| `progflow list` | Display all configured flows |
| `progflow new <name>` | Create a new flow (interactive or with flags) |
| `progflow edit <name>` | Open config in $EDITOR or update via flags |
| `progflow note <name>` | View the saved context note |
| `progflow status` | Show active flow and last note |
| `progflow delete <name>` | Delete a flow (with confirmation) |

### Command Details

#### `progflow on <name>`
Activates a workspace flow by:
- Verifying the configured directory exists
- Spawning the specified editor in the background (detached from current session)
- Opening all configured URLs in your default browser (Linux, macOS, or Termux)
- Writing process IDs to a lockfile for cleanup
- Displaying a summary of launched components (unless `--quiet` is used)

#### `progflow off [name]`
Deactivates a flow by:
- Automatically detecting the active flow if no name provided
- Reading process IDs from the lockfile
- Sending SIGTERM, waiting 3s, then SIGKILL to all tracked processes
- Prompting to save a context note (interactive mode only)
- Cleaning up the lockfile

#### `progflow new <name>`
Creates a new flow. If flags are provided, it runs non-interactively:
- `--dir <path>`: Working directory
- `--editor <cmd>`: Editor command
- `--urls <urls>`: Comma-separated URLs
- `--env <vars>`: Comma-separated KEY=VALUE pairs
- `--shell <path>`: Shell interpreter
- `--quiet`: Suppress success message

#### `progflow edit <name>`
- Opens the configuration file in `$EDITOR`.
- **New:** Use `--set-note "message"` to programmatically update the context note.

#### `progflow note <name>`
Displays the saved context note for a flow, or indicates "(no note saved)" if empty.

## One-Liner Support

Progflow is designed to be fully automatable. You can create, activate, and manage flows using single-line commands, which is perfect for AI agents and CI/CD pipelines.

### Examples:

**Create and start a flow in one go:**
```bash
progflow new myflow --dir $(pwd) --editor "code ." --quiet && progflow on myflow --quiet
```

**Update note and stop flow silently:**
```bash
progflow edit myflow --set-note "Finished feature X" --quiet && progflow off myflow --force --quiet
```

**Cleanup and delete:**
```bash
progflow off myflow --force --quiet && progflow delete myflow --force --quiet
```

Flows are stored in `~/.config/flow/<name>.json`:

```json
{
  "name": "dev",
  "directory": "/home/you/projects/myapp",
  "editorCmd": "nvim .",
  "urlList": ["https://github.com/user/myapp"],
  "shell": "/bin/bash",
  "env": {"NODE_ENV": "development"},
  "note": ""
}
```

### Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Unique identifier matching the filename |
| `directory` | string | No | Working directory for spawned processes |
| `editorCmd` | string | No | Command to launch your preferred editor |
| `urlList` | array | No | List of URLs to open automatically |
| `shell` | string | No | Shell interpreter (default: /bin/sh) |
| `env` | object | No | Environment variables as key-value pairs |
| `note` | string | No | Context note saved when stopping the flow |

## Uninstall

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/uninstall.sh | bash
```

---

## For Developers

### Overview

progflow is built in Rust with a focus on simplicity, performance, and portability. The architecture follows a modular design pattern with clear separation of concerns between CLI handling, configuration management, and platform-specific functionality.

### Requirements

- **Rust 1.70+** - The Rust toolchain must be installed on your system
- **Build tools** - `build-essential` on Debian/Ubuntu, or equivalent for your distribution
- **Git** - For cloning the repository during installation

### Building from Source

Clone the repository and build the release binary:

```bash
git clone https://github.com/Rehanasharmin/Progflow.git
cd Progflow
cargo build --release
```

The compiled binary will be located at `target/release/progflow`.

### Installing During Development

For local testing and development:

```bash
# Copy to your local bin directory
cp target/release/progflow ~/.local/bin/

# Or add the binary to your PATH
export PATH="$PWD:$PATH"
```

Verify the installation:

```bash
progflow --version
progflow --help
```

### Project Architecture

```
src/
├── main.rs          # CLI entry point using clap for argument parsing
├── config.rs        # FlowConfig struct, file I/O, lockfile management
├── error.rs         # Custom error types for robust error handling
├── platform.rs      # Platform detection and URL opening utilities
└── commands/
    ├── mod.rs       # Command module exports
    ├── on.rs        # Implementation of the 'on' command
    ├── off.rs       # Implementation of the 'off' command
    ├── list.rs      # Implementation of the 'list' command
    ├── edit.rs      # Implementation of the 'edit' command
    ├── new.rs       # Implementation of the 'new' command
    └── note.rs      # Implementation of the 'note' command
```

### Design Philosophy

- **No Async Runtime** - Uses blocking I/O throughout for minimal binary size and reduced complexity
- **Zero Dependencies** - Single static binary with no runtime requirements beyond the standard library
- **Cross-Platform** - Seamlessly works on Linux desktop and Termux on Android
- **Portable** - Configuration stored in XDG-compliant directory (`~/.config/flow/`)

### Platform Detection

The tool automatically detects Termux environments by checking:
1. The `$PREFIX` environment variable for `/data/data/com.termux`
2. The presence of `termux-open-url` in PATH

This enables Termux-specific URL opening behavior (using `termux-open-url` or `am start`) while defaulting to `xdg-open` on Linux systems.

### Lockfile Mechanism

When a flow is activated, process IDs are stored in `~/.config/flow/<name>.lock`. This enables proper cleanup when the flow is deactivated, ensuring no orphaned processes remain.

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | User error (invalid arguments, missing config) |
| 2 | I/O or JSON parsing error |

### Contributing

Contributions are welcome! Please ensure:
- Code follows Rust conventions and style
- All tests pass before submitting pull requests
- Binary builds without warnings on stable Rust

### Documentation

For detailed API documentation and advanced usage, visit: **[progflow Documentation](https://mintlify.wiki/Rehanasharmin/Progflow)**

## License

MIT License - See the LICENSE file for details.

---

Built with Rust for performance and portability.
