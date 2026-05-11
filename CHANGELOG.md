# Changelog

All notable changes to this project will be documented in this file.

## [0.1.3] - 2026-05-11

### Added
- Full non-interactive support for `new` and `edit` commands via CLI flags.
- `--set-note` flag for `edit` command to update flow notes programmatically.
- Detached process spawning for editors using `setsid` on Unix.
- macOS support for opening URLs using the `open` command.
- Integration test suite for non-interactive workflows.

### Improved
- Robust process termination: sends SIGTERM, waits 3s, then SIGKILL if still alive.
- Global `--quiet` flag now correctly suppresses output for all commands.
- Enhanced platform detection and command existence checks before opening URLs.
- Cleaner process spawning with stdout/stderr redirection to `/dev/null`.

### Fixed
- Fixed issue where `new` command would hang when piped into non-interactive shells.
- Fixed `gio open` command formatting in platform utilities.

## [0.1.2] - 2026-04-03

### Added
- `progflow status` - Show status of active flow
- `progflow delete <name>` - Delete a flow (with confirmation)
- `--json` flag for `list` command - JSON output for scripting
- `--verbose/-v` global flag - Detailed error output
- `--quiet/-q` global flag - Suppress output
- Timestamps on saved notes

### Improved
- Better error messages with recovery suggestions
- Config validation on load
- Improved process termination (safer than unsafe libc::kill)
- Version bump to 0.1.2

### Fixed
- Replaced unsafe `libc::kill` with std::process::Command
- Added config validation (empty names, invalid characters)

## [0.1.0] - 2026-02-05

### Added
- Initial release
- `progflow on` - Activate a flow
- `progflow off` - Deactivate a flow
- `progflow list` - List all flows
- `progflow new` - Create new flow
- `progflow edit` - Edit flow config
- `progflow note` - View saved notes
- Platform detection (Linux/Termux)
- Lockfile mechanism for process management
