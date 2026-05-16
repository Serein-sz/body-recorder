## 1. CLI Surface

- [x] 1.1 Add an advice command with an optional goal argument for `cut`, `maintain`, and `gain`, defaulting to `cut`
- [x] 1.2 Add an optional reference date argument that defaults to today
- [x] 1.3 Wire the command into the existing command dispatcher without changing existing commands

## 2. Trend Analysis

- [x] 2.1 Add trend analysis types for goal, data sufficiency, trend class, and advice result
- [x] 2.2 Fetch the required recent weight range relative to the reference date
- [x] 2.3 Compute medium-term trend in kilograms per week using smoothed recent data
- [x] 2.4 Compute short-term context separately from the medium-term recommendation signal
- [x] 2.5 Mark analysis as insufficient when records do not meet the minimum sample threshold

## 3. Advice Rules

- [x] 3.1 Map medium-term trend classes to conservative advice for fat loss, maintenance, and weight gain goals
- [x] 3.2 Return no adjustment recommendation when the trend signal is insufficient
- [x] 3.3 Flag unusually fast gain or loss with cautious messaging
- [x] 3.4 Avoid exact calorie targets, meal plans, diagnostic language, or medical advice in all recommendation text

## 4. Output

- [x] 4.1 Print a concise trend summary including reference date, records loaded, and kilograms-per-week trend when available
- [x] 4.2 Print goal interpretation and diet adjustment direction/intensity
- [x] 4.3 Print insufficient-data output that explains what data is missing instead of giving advice
- [x] 4.4 Keep output formatting consistent with existing CLI status and comparison output

## 5. Verification

- [x] 5.1 Add unit tests for trend calculation and data sufficiency boundaries
- [x] 5.2 Add unit tests for goal-specific advice mapping
- [x] 5.3 Add unit tests that verify insufficient data suppresses adjustment advice
- [x] 5.4 Run `cargo fmt`, `cargo test`, and `cargo check`
- [x] 5.5 Add parser coverage for omitted advice goal defaulting to `cut`
