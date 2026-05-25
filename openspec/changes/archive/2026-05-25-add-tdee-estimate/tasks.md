## 1. Domain Model and Calculation

- [x] 1.1 Add TDEE profile constants for male sex, birth date 2001-03-06, height 173 cm, and activity factor 1.60
- [x] 1.2 Add domain types for TDEE estimate result, calculation basis, and data-quality status
- [x] 1.3 Implement age calculation from reference date and birth date
- [x] 1.4 Implement 7-day average weight calculation with sample count
- [x] 1.5 Implement male Mifflin-St Jeor BMR and TDEE calculation
- [x] 1.6 Add unit tests for normal, low-sample, no-data, and birthday age boundary cases

## 2. Application Use Case

- [x] 2.1 Add a TDEE use case that parses the optional reference date
- [x] 2.2 Fetch weight records from the 7-day window ending on the reference date
- [x] 2.3 Return structured TDEE result data without treating low-sample estimates as errors
- [x] 2.4 Add use-case tests for repository date range and sparse-data behavior

## 3. CLI Surface

- [x] 3.1 Add a `tdee` CLI command with optional `--date`
- [x] 3.2 Route the command through the configured repository and TDEE use case
- [x] 3.3 Render estimated kcal/day, basis, sample count, and data-quality status
- [x] 3.4 Render no-recent-data output without a kcal/day estimate
- [x] 3.5 Add CLI parsing and output tests

## 4. TUI Surface

- [x] 4.1 Integrate TDEE into the Summary analysis view instead of adding a separate TDEE view
- [x] 4.2 Add TDEE load state to the TUI app state
- [x] 4.3 Load TDEE data through the TDEE use case when Summary opens or refreshes
- [x] 4.4 Render TDEE estimate value, basis, sample count, and data-quality status in Summary
- [x] 4.5 Render low-sample, no-data, loading, and error states in the Summary analysis panel
- [x] 4.6 Invalidate TDEE data after add, edit, or delete operations
- [x] 4.7 Add TUI state and rendering tests for Summary TDEE behavior

## 5. Verification

- [x] 5.1 Run `cargo fmt`
- [x] 5.2 Run `cargo test`
- [x] 5.3 Run `cargo check`
