## Why

Users who track weight and diet direction need a practical daily energy baseline to interpret calorie intake and weight trends. Adding a TDEE estimate gives the tool a concrete kcal/day reference while keeping advice grounded in recorded body weight data.

## What Changes

- Add a read-only TDEE estimate based on the latest 7-day average body weight.
- Use the user's known profile assumptions for the initial estimate: male, born 2001-03-06, height 173 cm, activity factor 1.60.
- Add a CLI command that prints the estimated TDEE in kcal/day with the calculation basis and data-quality status.
- Add TDEE visibility to the TUI analysis experience so the estimate is available alongside weight, BMI, trend, compare, and advice context.
- Handle sparse or missing recent weight data explicitly instead of silently producing unsupported values.

## Capabilities

### New Capabilities

- `tdee-estimate`: Estimate total daily energy expenditure in kcal/day from recent weight records and fixed personal profile assumptions.

### Modified Capabilities

- `ratatui-terminal-interface`: Display TDEE estimate information in the interactive terminal interface.

## Impact

- Domain calculations for age, recent weight averaging, BMR, and TDEE estimation.
- Application use case for loading recent weights and producing TDEE results.
- CLI command parsing and output rendering.
- TUI app state, loading, refresh, and rendering for the TDEE estimate.
- Tests for calculation, sparse data handling, CLI parsing/output, and TUI rendering state.
