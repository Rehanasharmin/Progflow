# Progflow Command Reference

This document provides a detailed breakdown of all available commands in Progflow. Whether you are setting up a new flow or managing an existing one, this guide will help you understand exactly what each command does and how to use it effectively.

## Core Commands

### `progflow on <name>`
Starts a named workspace flow.

*   **What it does:** Launches your defined editor, starts all background commands, and opens specified URLs in your browser.
*   **Flags:**
    *   `--skip-url-check`: Starts the flow without waiting for local servers to respond.
    *   `--edit-note`: Opens your default editor to update the flow's note before starting.
    *   `--note <text>`: Sets a temporary note for the current session.
    *   `--switch`: Automatically stops any currently active flow before starting this one.

### `progflow off [name]`
Stops a workspace flow.

*   **What it does:** Gracefully terminates all processes associated with the flow. If no name is provided, it stops the currently active flow.
*   **Flags:**
    *   `--force`: Skips the prompt to save a session note.
    *   `--note <text>`: Saves a final note for the session as the flow stops.

### `progflow status`
Shows the status of the active flow.

*   **What it does:** Displays which flow is currently running, how many processes are active, and any saved notes.
*   **Flags:**
    *   `--json`: Outputs the status in a structured JSON format, useful for scripts.

## Configuration Commands

### `progflow new <name>`
Creates a new workspace flow.

*   **What it does:** Scaffolds a new configuration file. It is interactive by default but can be fully automated with flags.
*   **Flags:**
    *   `--dir <path>`: Sets the working directory for the flow.
    *   `--editor <cmd>`: Sets the command to launch your IDE (e.g., "code .").
    *   `--urls <list>`: A comma-separated list of URLs to open.
    *   `--env <list>`: Comma-separated environment variables in KEY=VALUE format.
    *   `--shell <path>`: The path to the shell you want to use.
    *   `--cmd <cmd>`: A background command to start. Can be used multiple times.

### `progflow edit <name>`
Modifies an existing flow.

*   **What it does:** Opens the flow's configuration file in your editor or updates specific fields via flags.
*   **Flags:**
    *   Supports all flags available in the `new` command.
    *   `--set-note <text>`: Manually updates the persistent note for the flow.

### `progflow delete <name>`
Removes a flow configuration.

*   **What it does:** Deletes the JSON configuration file and any associated logs or locks.
*   **Alias:** `remove`
*   **Flags:**
    *   `--force`: Deletes the flow without asking for confirmation.

## Utility Commands

### `progflow list`
Lists all configured flows.

*   **What it does:** Displays a list of all flows you have created, indicating which ones are currently active.
*   **Flags:**
    *   `--json`: Outputs the list in JSON format.

### `progflow logs <name>`
Shows the logs for a flow.

*   **What it does:** Displays the output (stdout and stderr) of all background commands associated with the specified flow.

### `progflow stats <name>`
Displays usage analytics.

*   **What it does:** Shows how many times you have used a flow and the total time you have spent working in it.

### `progflow note <name>`
Displays the last saved note for a flow.

### `progflow aliases`
Generates shell aliases.

*   **What it does:** Creates POSIX-compliant aliases for all your flows. You can add `eval "$(progflow aliases)"` to your shell profile to use `flow-<name>` shortcuts.

### `progflow update`
Updates Progflow.

*   **What it does:** Checks for a newer version on GitHub and updates the tool if one is available.
