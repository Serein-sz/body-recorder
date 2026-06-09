## Why

The TUI add-record form currently accepts an optional date only through manual typing, which makes selecting nearby dates slower than the rest of the keyboard-driven workflow. Users should be able to adjust the add date with simple arrow-key controls while staying in the form.

## What Changes

- Add left and right arrow key behavior for selecting the date in the TUI add-record form.
- Treat the blank add-form date as the current TUI reference date when arrow selection begins, then fill the selected date as `YYYY-MM-DD`.
- Keep the behavior scoped to add-record input so normal-mode record navigation and analysis view behavior remain unchanged.
- Preserve existing manual date typing, validation, and save behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `ratatui-terminal-interface`: Add keyboard date selection behavior to the add-record form.

## Impact

- Affected code: `src/presentation/tui/action.rs`, `src/presentation/tui/event.rs`, `src/presentation/tui/app.rs`, and `src/presentation/tui/ui.rs`.
- Tests should cover key mapping, add-form date adjustment, invalid typed-date handling, and rendered help text.
- No storage, API, dependency, or database changes.
