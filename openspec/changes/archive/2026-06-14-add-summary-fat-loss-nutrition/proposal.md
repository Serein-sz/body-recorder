## Why

The Summary analysis view already gives a high-level picture of weight trend, BMI, and estimated daily energy needs, but it does not translate the user's current body weight into practical fat-loss macronutrient targets. Adding this information keeps the most useful daily nutrition guidance in the first analysis panel the user sees.

## What Changes

- Add fat-loss macronutrient target calculations from body weight and weekly training duration.
- Use the user's current training duration band of 6-7 hours per week as the default profile for the Summary view.
- Display daily carbohydrate, protein, and fat targets in the TUI Summary analysis panel.
- Prefer the recent 7-day average weight as the calculation basis, with a latest-record fallback when the average is unavailable.
- Keep the output estimate-oriented and avoid presenting the targets as medical advice or a precise prescription.

## Capabilities

### New Capabilities

- `fat-loss-nutrition-targets`: Defines fat-loss macronutrient target calculation from body weight and weekly training duration.

### Modified Capabilities

- `ratatui-terminal-interface`: Summary analysis renders fat-loss macronutrient targets alongside existing trend, BMI, chart, and TDEE context.

## Impact

- Affects domain nutrition/stat calculation code.
- Affects TUI Summary rendering and related TUI tests.
- Adds OpenSpec coverage for nutrition target behavior and Summary presentation.
- No storage schema, network API, or CLI command behavior changes are expected.
