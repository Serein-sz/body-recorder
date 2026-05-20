## Why

Weight records currently show kilogram values and trend movement, but they do not give height-adjusted context. Adding BMI gives the user a quick way to understand each weight and recent averages relative to a fixed personal height of 1.73 meters.

## What Changes

- Add BMI calculation based on the fixed height `1.73m`.
- Display BMI values alongside relevant weight records, averages, comparison points, and advice summaries.
- Display the adult BMI category standard for each BMI value: underweight, normal, overweight, or obesity.
- Render BMI category labels with status colors so normal values are green, cautionary values are yellow, and obesity is red.
- Keep the change local to calculation and presentation; no storage schema or command argument changes are required.

## Capabilities

### New Capabilities

- `body-mass-index-display`: BMI calculation, category classification, and CLI/TUI presentation behavior for weight records and analysis summaries.

### Modified Capabilities

- `ratatui-terminal-interface`: TUI record and analysis views should include BMI values and colored BMI category labels where weight values or averages are shown.
- `trend-based-diet-advice`: Advice output should include BMI context for short-term average weight without changing the conservative trend-based recommendation model.
- `historical-weight-comparison`: Comparison output should include BMI context for baseline and historical average weights.

## Impact

- Affected code: `src/stats.rs`, `src/output.rs`, `src/tui/ui.rs`, and tests around output or statistics formatting.
- No database schema changes.
- No new dependencies expected; existing ANSI and Ratatui styling should be reused.
- Existing commands remain compatible, but user-visible output gains BMI columns or labels.
