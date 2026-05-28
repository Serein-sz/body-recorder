## 1. Chart Data Helpers

- [x] 1.1 Add helper logic to order visible weight records oldest-to-newest for charting
- [x] 1.2 Keep chart helper logic scoped to Summary weight charts
- [x] 1.3 Add helper logic to produce Summary chart points and y-axis bounds with padding
- [x] 1.4 Add unit tests for chart data ordering, insufficient data, and bounds

## 2. Recent Records Panel

- [x] 2.1 Keep the Recent records panel text-only
- [x] 2.2 Preserve existing record rows and selection styling
- [x] 2.3 Do not reserve chart space in the Recent records panel
- [x] 2.4 Add rendering tests for text-only Recent records behavior

## 3. Summary Weight Chart

- [x] 3.1 Split the Summary analysis panel into text and chart regions when enough space is available
- [x] 3.2 Render a weight trend chart from recent records in the Summary analysis view
- [x] 3.3 Preserve Summary trend, BMI, and TDEE text when the chart is shown
- [x] 3.4 Keep Summary text-only when fewer than two records or insufficient space are available
- [x] 3.5 Add rendering tests for Summary chart presence and text-only fallback

## 4. Non-Chart Panels

- [x] 4.1 Verify Compare remains table-first without chart rendering
- [x] 4.2 Verify Advice remains text-first without chart rendering
- [x] 4.3 Verify Target remains text-first without chart rendering

## 5. Verification

- [x] 5.1 Run `cargo fmt`
- [x] 5.2 Run `cargo test`
- [x] 5.3 Run `cargo check`
