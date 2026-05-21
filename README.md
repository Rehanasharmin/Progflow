# 🚀 Progflow: The Context-Aware Workspace Manager

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Termux-lightgrey.svg?style=flat-square)](https://github.com/Rehanasharmin/Progflow)

**Progflow** is a high-performance, context-aware productivity tool built in Rust. It automates your development environment by orchestrating editors, background services, and browser sessions into a single "flow." Switch between projects instantly without losing your mental state or manual configuration overhead.

> "Stop setting up, start coding. Progflow remembers where you left off so you don't have to."

---

## ✨ Key Features

- **⚡ Instant Activation**: Launch your IDE, local dev servers, and documentation with one command.
- **📝 Persistent Context Notes**: Save progress notes when stopping a flow; see them automatically when you return.
- **🔄 Parallel Start Commands**: Run databases, builders, or watchers in the background and track their PIDs.
- **🕵️ URL Readiness Check**: Automatically verifies local services (e.g., `localhost:3000`) before opening the browser.
- **🖥️ OS-Aware Intelligence**: Tailored workflow tips for **Linux**, **macOS**, and **Termux**.
- **📋 Integrated Logs**: Inspect the output of all background start commands with `progflow logs`.
- **🤖 Automation First**: Full "one-liner" support with flags for all fields—perfect for CI/CD and AI agents.

---

## 📸 Terminal Experience

### Starting a Flow
```text
$ progflow on my-project
💡 Tip: Use 'progflow logs my-project' to see the output of your background start commands.
✓ flow 'my-project' started — editor, 2 start commands, 3 urls
Previous note: [2024-05-21 14:30] Finished the login API, need to start on auth middleware.
```

### Checking Status
```text
$ progflow status
Active flow: my-project
Running processes: 3
Note saved: [2024-05-21 14:30] Finished the login API...
```

---

## 🚀 Getting Started

### Installation

Install instantly via our optimized script:
```bash
curl -sSL https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh | bash
```

Or build from source for maximum performance:
```bash
git clone https://github.com/Rehanasharmin/Progflow.git
cd Progflow
cargo build --release
cp target/release/progflow ~/.local/bin/
```

### Quick Commands

| Command | Action |
| :--- | :--- |
| `progflow new <name>` | Scaffold a new workspace flow |
| `progflow on <name>` | Activate your environment & background tasks |
| `progflow status` | See active flow and running process count |
| `progflow off` | Cleanly terminate processes & save a note |
| `progflow logs <name>` | View logs from background services |
| `progflow list` | List all flows with activity indicators |

---

## 💡 Why Progflow?

### 1. Unified Development Environments
Define your entire stack in a simple JSON config. Launch VS Code, a React dev server, and your PostgreSQL database simultaneously.
```bash
progflow new webapp --dir ~/code/webapp --editor "code ." --cmd "npm run dev" --urls "http://localhost:3000"
```

### 2. Effortless Context Switching
Switching from a backend bug to a frontend feature? `progflow off` kills the backend PIDs and saves your spot. `progflow on frontend` restores the new context instantly.

### 3. Smarter Automation
Designed for power users and AI agents. No interactive prompts required.
```bash
progflow new bot-flow --dir $(pwd) --env "API_KEY=secret" --quiet
```

---

## 🛠️ Configuration Schema

Flows are stored as human-readable JSON in `~/.config/flow/<name>.json`.

```json
{
  "name": "project-x",
  "directory": "/home/user/dev/project-x",
  "editorCmd": "nvim .",
  "urlList": ["https://github.com", "http://localhost:8080"],
  "shell": "/bin/zsh",
  "env": { "DEBUG": "true" },
  "startCommands": [
    { "command": "docker-compose up", "background": true },
    { "command": "npm run watch", "background": true }
  ],
  "lastNote": "[2024-05-21] Refactored the core parser."
}
```

---

## 🐧 Platform Compatibility

- **Linux**: Full support with `xdg-open` and desktop integration tips.
- **macOS**: Native `open` command support and Automator/Shortcut tips.
- **Termux**: Optimized for mobile dev with `termux-open-url` and `termux-boot` support.

---

## 🤝 Contributing

Contributions are what make the open-source community an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

<p align="center">
  Built with ❤️ in Rust for developers who value their time.
  <br>
  <b><a href="https://progflowcli.netlify.app">Official Website</a></b> • <b><a href="https://github.com/Rehanasharmin/Progflow/issues">Report a Bug</a></b>
</p>
