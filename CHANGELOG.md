# Changelog

All notable changes to this project will be documented in this file.

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

## [0.1.0] - 2025-01-01

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
