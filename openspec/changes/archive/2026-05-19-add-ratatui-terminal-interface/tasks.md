## 1. CLI Entry And Dependencies

- [x] 1.1 Add compatible Ratatui and Crossterm dependencies to `Cargo.toml`.
- [x] 1.2 Add a `Tui` subcommand to the Clap command enum without changing existing command arguments.
- [x] 1.3 Route `Commands::Tui` through the command dispatcher by constructing the configured repository and calling a new TUI entry function.
- [x] 1.4 Add the `src/tui/` module structure for app state, actions, event handling, rendering, and terminal lifecycle.

## 2. Terminal Lifecycle

- [x] 2.1 Implement terminal setup for raw mode and alternate screen using Crossterm.
- [x] 2.2 Implement cleanup that leaves alternate screen and disables raw mode on normal exit.
- [x] 2.3 Ensure cleanup also runs when the TUI returns an unrecoverable error.

## 3. TUI State And Events

- [x] 3.1 Define TUI state for records, selected row, loading/error status, current screen or input mode, and pending operation state.
- [x] 3.2 Define TUI actions for refresh, add, edit, delete, confirm/cancel, navigation, text input, and quit.
- [x] 3.3 Map Crossterm key events to TUI actions, filtering to supported key press events.
- [x] 3.4 Implement state transitions for navigation, mode changes, input editing, and error dismissal without storage access.

## 4. Data Operations

- [x] 4.1 Load recent records through `WeightRepository` when the TUI starts.
- [x] 4.2 Implement refresh through the existing list use case while preserving previous records on refresh failure.
- [x] 4.3 Implement add from TUI input through the existing add use case and refresh or update displayed records after success.
- [x] 4.4 Implement edit for the selected record through the existing update use case and refresh or update displayed records after success.
- [x] 4.5 Implement delete confirmation for the selected record through the existing delete use case and refresh or update displayed records after success.
- [x] 4.6 Display validation and storage errors in TUI state without sending invalid data to storage.

## 5. Rendering

- [x] 5.1 Render the main TUI layout with recent records, selected row state, status/error area, and command affordances.
- [x] 5.2 Render empty-state, loading, and error states for the records area.
- [x] 5.3 Render add/edit input states with date and weight fields as needed.
- [x] 5.4 Render delete confirmation for the selected record.
- [x] 5.5 Render concise trend or advice context from existing domain calculations, including insufficient-data state.

## 6. Tests And Verification

- [x] 6.1 Add tests for key event to action mapping.
- [x] 6.2 Add tests for pure TUI state transitions using fake records.
- [x] 6.3 Add tests for add/edit/delete validation paths that prove invalid input does not call storage.
- [x] 6.4 Add render tests for loaded, empty, loading, and error screen states using a Ratatui test backend or equivalent stable assertions.
- [x] 6.5 Run `cargo fmt`.
- [x] 6.6 Run `cargo test`.
- [x] 6.7 Run `cargo check`.

## 7. Rounded Borders

- [x] 7.1 Capture rounded border behavior in the TUI spec.
- [x] 7.2 Render all TUI panels and overlays with Ratatui rounded border type.
- [x] 7.3 Add render coverage that verifies rounded border corners are present.
- [x] 7.4 Run `cargo fmt`.
- [x] 7.5 Run `cargo test`.
- [x] 7.6 Run `cargo check`.
