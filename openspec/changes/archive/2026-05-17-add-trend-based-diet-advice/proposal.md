## Why

Body weight records are useful for more than historical comparison: users want to know whether their recent trend calls for a diet adjustment. A trend-based advice command can turn existing weight data into cautious, actionable guidance while avoiding overreacting to noisy day-to-day fluctuations.

## What Changes

- Add a diet advice capability that analyzes recent weight trends from recorded weights.
- Report short-term and medium-term trend signals, including data sufficiency and confidence.
- Interpret trends against a goal: fat loss, maintenance, or weight gain, defaulting to fat loss when omitted.
- Provide conservative diet adjustment guidance as direction and intensity, not medical or prescriptive meal planning.
- Avoid giving adjustment advice when there is insufficient or unreliable recent data.
- No breaking changes to existing weight recording, listing, updating, deleting, or comparison commands.

## Capabilities

### New Capabilities

- `trend-based-diet-advice`: Analyze recent weight trends and produce goal-aware diet adjustment guidance.

### Modified Capabilities

None.

## Impact

- CLI: add a command for requesting diet advice with an optional goal and reference date.
- Statistics: add trend analysis over recent weight windows, including rate of change and data sufficiency.
- Output: add human-readable advice text with clear status, interpretation, and recommended adjustment direction.
- Tests: add unit coverage for trend classification, data sufficiency handling, and goal-specific advice mapping.
- Storage/API: reuse existing Supabase weight records; no schema change is expected for the MVP.
