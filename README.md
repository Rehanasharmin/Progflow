## Progflow: Context-Aware Workspace Orchestration Utility

[Official Website](https://progflowcli.netlify.app) | [Source Code](https://github.com/Rehanasharmin/Progflow) | [Issue Tracker](https://github.com/Rehanasharmin/Progflow/issues)

---

## Metadata and Build Status

![Rust Version](https://img.shields.io/badge/Rust-1.70%2B-orange.svg?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)
![Platform Support](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Termux-lightgrey.svg?style=flat-square)
![Code Style](https://img.shields.io/badge/Code%20Style-Rustfmt-brightgreen.svg?style=flat-square)

---

## Executive Summary

Progflow is a specialized command-line interface (CLI) utility architected in Rust for the rigorous orchestration of development environments. It facilitates low-overhead context switching by encapsulating editor configurations, persistent environment variables, network endpoints, and detached background processes into declarative, single-command "flows." By automating the transactional initialization and graceful termination of these environments, Progflow minimizes cognitive switching costs and resource leaks during complex multi-project engineering workflows.

---

## Core Architecture and Capabilities

* **Stateful Workspace Encapsulation**: Single-command creation, storage, and invocation of isolated development profiles.
* **Asynchronous Process Supervision**: Spawns and tracks background daemon processes, internalizing PID mapping and execution telemetry.
* **Sequential Resource Verification**: Employs synchronous TCP handshake validation on local service ports before executing external web browser calls.
* **Context Persistence Layer**: Captures runtime developer notes on session teardown and injects them back into the workspace state upon re-initialization.
* **Tiered Process Teardown**: Ensures clean host environment state through systematic signal propagation (`SIGTERM` progressing to fallback `SIGKILL`).
* **POSIX Cross-Platform Engine**: Native platform translation layer adapting behavior across standard Linux, macOS, and Termux terminal environments.
* **Pipeline Non-Interactive Abstraction**: Standardized JSON input/output flags enabling integration with custom wrappers, cron automation, or continuous integration environments.

---

## Technical Specifications and Guardrails

### Race-Condition and Double-Activation Prevention
Progflow writes and validates atomic locks within its persistent runtime path to prevent simultaneous initialization of duplicate configurations. This mitigates risks involving state collision, conflicting port bindings, and detached process orphaning.

### Lifecycle Management Sequence

```
[progflow on]  ──> Read Config ──> Export Env ──> Launch Shell/Editor ──> Spawn Daemons ──> Verify Ports ──> Open URLs
[progflow off] ──> Fetch PIDs  ──> Send SIGTERM ──> Grace Period Wait ──> Fallback SIGKILL ──> Save Context Note
```

---

## System Installation

### Pre-compiled Script (Recommended)
Execute the automated wrapper script to dynamically resolve architecture, download the corresponding binary release, and append it to your persistent user path:

```bash
curl -sSL [https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh](https://raw.githubusercontent.com/Rehanasharmin/Progflow/master/install.sh) | bash

```
### Compilation From Source
Ensure cargo and the Rust toolchain (v1.70+) are configured within your shell path. Clone and compile the release target manually:
```bash
git clone [https://github.com/Rehanasharmin/Progflow.git](https://github.com/Rehanasharmin/Progflow.git)
cd Progflow
cargo build --release
cp target/release/progflow ~/.local/bin/

```
## Comprehensive Command Reference
### Global Flags
| Flag | Long Alternative | Functional Definition |
|---|---|---|
| -v | --verbose | Elevates standard tracing log level to output explicit debugging telemetry. |
| -q | --quiet | Suppresses default stdout notifications to keep script interfaces concise. |
### Configuration and Execution Subcommands
#### progflow new <name>
Instantiates an isolated workspace profile definition.
 * --dir <path>: Specifies absolute path to target root workspace directory.
 * --editor <cmd>: Designates preferred terminal editor or external IDE binary hook.
 * --urls <list>: Comma-delimited sequence of local/remote web dependencies.
 * --env <list>: Comma-delimited structural properties formatted explicitly as KEY=VALUE.
 * --shell <path>: Defines target execution shell interpreter path (e.g., /bin/zsh).
 * --start-commands <json>: Pass arrays of objects detailing system commands to daemonize.
 * --cmd <cmd>: Inject a supplementary foreground or background dependency script.
 * --cmd-dir <path>: Sets specific working runtime directory for matching index --cmd.
 * --cmd-bg <bool>: Explicitly details if corresponding index command detaches as daemon.
#### progflow edit <name>
Modifies parameters within an active workspace configuration file. Accepts all technical flag inputs initialized during progflow new updates.
 * --set-note <text>: Overwrites the internal documentation field directly without spawning an interactive terminal buffer.
#### progflow on <name>
Deploys and initializes target workspace structures.
 * --skip-url-check: Forces application execution to bypass verification loops waiting on local TCP ports.
 * --edit-note: Directly drops user into default editor context to read/write log files before firing initialization blocks.
 * --note <text>: Injects temporary runtime data directly into current session logs via arguments.
#### progflow off [name]
Signals termination across all active PIDs mapped to the targeted environment.
 * --force: Hard-closes processes, avoiding contextual markdown or terminal note dialog prompts.
 * --note <text>: Logs a final transaction summary before breaking execution context.
#### progflow status
Exposes system diagnostics regarding active workspace instances.
 * --json: Transforms raw human-readable stdout data into standard machine-parseable JSON properties.
#### progflow list
Iterates over known configurations on disk, highlighting current execution status flags.
 * --json: Outputs raw array metadata for downstream tooling ingest.
#### progflow log <name>
Tails unified stdout and stderr execution sequences preserved from detached daemon nodes tracking inside the context workspace.
#### progflow note <name>
Dumps text blobs logged during the close phase of the previous session directly into terminal stdout.
#### progflow update
Pulls runtime binary checksums against downstream releases, updating regional bin configurations seamlessly.
#### progflow delete <name> *(Alias: remove)*
Purges workspace flow target files from local disk state.
 * --force: Bypasses default console validation safety confirmations.
## Storage Schema and Configuration Specification
Environment state schemas are serialized and maintained as standard JSON structures within the persistent configuration directory paths (~/.config/flow/).
```json
{
  "name": "enterprise-service",
  "directory": "/opt/dev/enterprise-service",
  "editorCmd": "nvim .",
  "urlList": [
    "[https://internal-docs.local](https://internal-docs.local)",
    "http://localhost:8080"
  ],
  "shell": "/bin/zsh",
  "env": {
    "NODE_ENV": "production",
    "RUST_BACKTRACE": "1"
  },
  "startCommands": [
    {
      "command": "docker-compose up -d",
      "background": true
    },
    {
      "command": "cargo watch -x run",
      "background": true
    }
  ],
  "lastNote": "[2026-05-23] Finalized routing tables and resolved memory leak on telemetry loop."
}

```
## Operating System Interoperability
Progflow wraps native kernel abstraction endpoints gracefully based on environmental target compilation flags:
 * **Linux**: Directs network routing hooks and targets graphical browser sessions utilizing standard system xdg-open APIs.
 * **macOS**: Coordinates binary hooks directly into standard framework execution pipelines via the underlying native Apple open execution engine.
 * **Termux**: Custom binary layer adapting architecture hooks to process execution rules through custom termux-open-url bindings and standard Android task managers.
## Contribution Guidelines
Contributions to the codebase require strict adherence to standard idiom paradigms. Maintain zero compilation warnings, enforce cargo clippy and cargo fmt standards across patches, and verify all regression tests execute without errors inside continuous deployment simulations before initiating pull requests.
## License Information
Distributed transparently under the terms of the MIT open-source license agreements. Review the localized LICENSE file for full textual details.
