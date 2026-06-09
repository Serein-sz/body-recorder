## 1. Input Actions

- [x] 1.1 Add explicit previous-date and next-date actions for TUI input handling
- [x] 1.2 Map left and right arrow key events to the new date-selection actions
- [x] 1.3 Add event mapping tests for left and right arrow keys

## 2. Add Form Date Selection

- [x] 2.1 Implement one-day date adjustment in add mode when the date field is active
- [x] 2.2 Use the TUI reference date as the base when the add date field is blank
- [x] 2.3 Leave invalid typed add dates unchanged when arrow selection is attempted
- [x] 2.4 Ensure arrow date-selection actions do not affect normal mode, edit mode, or the add form weight field

## 3. User Feedback

- [x] 3.1 Update add/edit form help text to expose left/right date selection only where appropriate
- [x] 3.2 Add or update TUI rendering tests for the date-selection help text

## 4. Verification

- [x] 4.1 Add app-state tests covering blank-date, valid-date, invalid-date, and scoped/no-op behavior
- [x] 4.2 Run `cargo fmt`
- [x] 4.3 Run `cargo check`
- [x] 4.4 Run `cargo test`
- [x] 4.5 Run `openspec status --change add-tui-date-arrow-selection`
