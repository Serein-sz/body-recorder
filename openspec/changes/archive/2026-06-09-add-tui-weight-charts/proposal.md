## Why

The TUI already shows body weight records and analysis text, but users must visually scan numbers to understand recent movement. Lightweight charts can make weight trends easier to read at a glance while avoiding charts in panels where they would add noise.

## What Changes

- Add a weight trend chart to the Summary analysis view using recent records already loaded by the TUI.
- Keep the recent records panel text-only so it stays focused on record scanning and editing.
- Keep Compare, Advice, Target, and TDEE text-first unless a chart would clearly improve comprehension.
- Hide chart areas gracefully when there is not enough data or terminal space.
- Use Ratatui's built-in chart widgets and avoid adding new dependencies.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `ratatui-terminal-interface`: Add an appropriate Summary weight trend chart while preserving keyboard behavior and existing analysis views.

## Impact

- TUI rendering layout for the Summary analysis panel.
- TUI helper logic for converting weight records into ordered chart data.
- Tests for chart visibility, no-data behavior, and ensuring non-chart panels remain readable.
