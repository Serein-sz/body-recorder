## 1. Domain BMI Calculation

- [x] 1.1 Add fixed height, BMI calculation, optional BMI helper, and `BmiCategory` classification to `src/stats.rs`.
- [x] 1.2 Add unit tests for BMI formula, rounding expectations, category boundary values, and missing average handling.

## 2. CLI Rendering

- [x] 2.1 Update weight list output to show BMI value and colored BMI category for each record.
- [x] 2.2 Update comparison output to show BMI context for the recent baseline and each available historical comparison point.
- [x] 2.3 Update advice output to show BMI context for the short-term average when available.
- [x] 2.4 Add or update output tests covering BMI labels and category color behavior.

## 3. TUI Rendering

- [x] 3.1 Update recent records rendering to include BMI value and colored category label.
- [x] 3.2 Update Summary view lines to include BMI context for the short-term average when available.
- [x] 3.3 Update Compare view lines to include BMI context for baseline and historical averages when available.
- [x] 3.4 Update Advice view lines to include BMI context for the short-term average when available.

## 4. Verification

- [x] 4.1 Run `cargo fmt`.
- [x] 4.2 Run `cargo check`.
- [x] 4.3 Run `cargo test`.
- [x] 4.4 Run `openspec status --change add-bmi-display` and confirm the change is ready for implementation progress tracking.
