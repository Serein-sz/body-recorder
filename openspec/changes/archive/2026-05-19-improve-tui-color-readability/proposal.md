## Why

The TUI now has multiple panels and analysis states, but most text is rendered with the same terminal foreground, making status, selection, and analysis signals harder to scan. A small semantic color system can improve readability without changing workflows or relying on decoration.

## What Changes

- Add semantic colors for panel borders, panel titles, active tabs, selected records, status messages, loading states, and errors.
- Add semantic colors for compare values such as deltas and value sources.
- Add semantic colors for advice values such as data status, trend class, recommendation intensity, and caution.
- Keep textual labels such as `direct`, `filled`, `missing`, `error`, and `no data` so meaning is not conveyed by color alone.
- Keep the existing keyboard interactions, layout, rounded borders, and analysis views unchanged.

## Capabilities

### New Capabilities

### Modified Capabilities

- `ratatui-terminal-interface`: Add readable semantic color behavior for TUI panels, statuses, selected content, compare analysis, and advice analysis.

## Impact

- Affected code: `src/tui/ui.rs` and focused render tests.
- Affected behavior: `br tui` becomes easier to scan through semantic styling; no command arguments, storage behavior, or workflows change.
- Affected dependencies: none expected.
