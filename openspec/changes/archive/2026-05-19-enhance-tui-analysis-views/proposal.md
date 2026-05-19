## Why

The current TUI shows recent records and a compact trend summary, but the richer `compare` and `advice` command insights still require leaving the interactive interface. Bringing those analyses into the TUI keeps the record list as context while making historical comparison and diet guidance easier to review during normal use.

## What Changes

- Add TUI analysis views for Summary, Compare, and Advice.
- Keep the recent records list visible while switching the right-side analysis panel between views.
- Reduce the records panel width so compare/advice content has more room.
- Render compare output as a concise table with baseline, historical periods, deltas, and value source.
- Render advice output as a structured panel with goal, data status, trend class, interpretation, and recommendation.
- Add keyboard controls for switching analysis views, refreshing the active analysis, and changing the advice goal.
- Preserve existing record management workflows and existing non-TUI CLI command behavior.

## Capabilities

### New Capabilities

### Modified Capabilities

- `ratatui-terminal-interface`: Add analysis-view behavior, compare/advice rendering, keyboard navigation between views, advice goal selection, and revised split-panel layout.

## Impact

- Affected code: `src/tui/` state, actions, rendering, tests, and use-case integration.
- Affected behavior: `br tui` gains analysis views and a wider analysis panel; existing add/edit/delete/refresh record workflows remain available.
- Affected dependencies: none expected.
- Affected systems: Supabase remains accessed through existing use cases and `WeightRepository`.
