## 1. Comparison Range

- [x] 1.1 Update `comparison_range` to return `reference_date - 365 days` through `reference_date`
- [x] 1.2 Update use-case tests to verify `compare` requests only the one-year inclusive range

## 2. Domain Calculation

- [x] 2.1 Add comparison value source metadata for direct, filled, and missing historical points
- [x] 2.2 Keep recent baseline calculation based on direct records only
- [x] 2.3 Implement smoothed fill for empty historical target windows using nearest surrounding records inside the loaded range
- [x] 2.4 Preserve missing comparison points when direct records and surrounding fill records are unavailable
- [x] 2.5 Calculate deltas from the direct recent baseline to either direct or filled historical values

## 3. Output Formatting

- [x] 3.1 Update comparison rendering to distinguish direct, filled, and missing values
- [x] 3.2 Keep existing comparison command arguments and concise terminal output shape

## 4. Tests and Verification

- [x] 4.1 Add unit tests for direct historical averages taking precedence over fills
- [x] 4.2 Add unit tests for smoothed filled historical values
- [x] 4.3 Add unit tests for unfillable missing values and missing baseline behavior
- [x] 4.4 Run `cargo fmt`, `cargo test`, and `cargo check`
