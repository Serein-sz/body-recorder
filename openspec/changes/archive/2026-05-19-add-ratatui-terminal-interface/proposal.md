## Why

The current application is a command-oriented CLI, which works well for scripted actions but makes day-to-day review and correction of weight records repetitive. A Ratatui terminal interface can provide an interactive view for recent records and common record-management actions while preserving the existing CLI commands for automation.

## What Changes

- Add an interactive terminal interface exposed through a new `br tui` command.
- Let users view recent weight records, refresh data, add a new weight, edit an existing weight, and delete a selected record from the TUI.
- Show lightweight trend and advice context in the TUI using existing domain calculations.
- Keep existing CLI command behavior unchanged.
- Add Ratatui/Crossterm-based terminal rendering and event handling dependencies.
- Add focused tests for TUI state transitions, event mapping, and key rendered states without making real network calls.

## Capabilities

### New Capabilities

- `ratatui-terminal-interface`: Covers the interactive terminal UI behavior, including entry, navigation, record actions, status/error display, and terminal cleanup.

### Modified Capabilities

- `maintainable-application-architecture`: Extend architecture expectations so the TUI is treated as a presentation layer that reuses existing use cases, domain logic, and storage interfaces.

## Impact

- Affected code: `src/cli.rs`, `src/commands.rs`, new TUI modules under `src/tui/`, and tests.
- Affected dependencies: add Ratatui and Crossterm-compatible terminal event/rendering crates.
- Affected behavior: new `br tui` command only; existing `init`, `schema`, `add`, `list`, `update`, `delete`, `compare`, and `advice` commands keep their current arguments and meanings.
- Affected systems: Supabase remains the production storage backend through the existing `WeightRepository` boundary.
