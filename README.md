# progflow

A context-aware workspace manager for Linux and Termux.

## What is it?

progflow lets you define "flows" - workspace configurations that automatically launch your editor, open URLs, and set up your environment with a single command. When you stop a flow, it saves your context as a note for next time.

## Installation

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash
```

## Quick Start

```bash
progflow new dev
# Follow the prompts:
# - Working directory: /home/you/projects/myapp
# - Editor command: nvim .
# - URLs to open: https://github.com, https://docs.rs
# - Shell: /bin/bash
# - Env vars (optional): NODE_ENV=development

progflow on dev      # Start working
progflow off dev    # Stop and save note
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

### 2. Multi-Service Tasks
Open all related tools at once:
```bash
progflow new monitoring
# directory: /home/user
# editor: (empty)
# URLs: grafana.local, prometheus.local, alertmanager.local
```

### 3. Documentation Workflow
Quick access to documentation:
```bash
progflow new docs
# directory: ~/docs
# editor: nvim .
# URLs: rust-lang.org, docs.rs, github.com
```

### 4. Context Tracking
Never lose track of what you were doing:
```bash
progflow off myflow
# Save a context note? [y/N]: y
# Enter note: Was debugging auth issue in JWT middleware

# Next time:
progflow note myflow
# Output: Was debugging auth issue in JWT middleware
```

### 5. Termux (Android)
Works the same on Termux - uses `termux-open-url` instead of `xdg-open`:
```bash
progflow new termux-work
# directory: ~/projects
# editor: termux-editor .
# URLs: github.com
```

## Commands

| Command | Description |
|---------|-------------|
| `progflow on <name>` | Activate a flow |
| `progflow off [name]` | Deactivate flow (saves note) |
| `progflow list` | List all flows |
| `progflow new <name>` | Create new flow |
| `progflow edit <name>` | Edit config in $EDITOR |
| `progflow note <name>` | View flow's saved note |

## Configuration

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

## Uninstall

```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/uninstall.sh | bash
```

---

## For Developers

### Requirements
- Rust 1.70+
- `build-essential`

### Build
```bash
cargo build --release
./target/release/progflow --help
```

### Install during development
```bash
cp target/release/progflow ~/.local/bin/
```

### Project Structure
```
src/
├── main.rs          # CLI entry (clap)
├── config.rs        # FlowConfig, lockfile I/O
├── error.rs         # AppError enum
├── platform.rs      # is_termux(), URL opener
└── commands/
    ├── on.rs        # progflow on
    ├── off.rs       # progflow off  
    ├── list.rs      # progflow list
    ├── edit.rs      # progflow edit
    ├── new.rs       # progflow new
    └── note.rs      # progflow note
```

### Design Notes
- No async: blocking I/O for smaller binary
- Platform detection: `$PREFIX` or `termux-open-url` presence
- Lockfiles: PIDs in `~/.config/flow/<name>.lock`
- Exit codes: 1=user error, 2=IO/JSON error

## License

MIT
