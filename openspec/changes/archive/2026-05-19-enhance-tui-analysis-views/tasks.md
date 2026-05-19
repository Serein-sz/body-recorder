## 1. Analysis State

- [x] 1.1 Add TUI analysis view state for Summary, Compare, and Advice.
- [x] 1.2 Add independent load state for Compare and Advice analysis results.
- [x] 1.3 Add advice goal state defaulting to fat loss.
- [x] 1.4 Invalidate Compare and Advice load states after add, edit, or delete succeeds.

## 2. Actions And Data Loading

- [x] 2.1 Add keyboard action for rotating the analysis view.
- [x] 2.2 Add keyboard action for changing the advice goal in normal mode.
- [x] 2.3 Load Compare view data through `use_cases::compare` when the view is opened or refreshed.
- [x] 2.4 Load Advice view data through `use_cases::advice` for the selected goal when the view is opened, refreshed, or goal changes.
- [x] 2.5 Keep Summary refresh behavior tied to recent-record refresh.
- [x] 2.6 Prevent analysis view and advice goal controls from changing state while record input or delete confirmation modes are active.

## 3. Rendering

- [x] 3.1 Change the split layout so the records panel is narrower than the analysis panel.
- [x] 3.2 Render analysis view labels or tabs in the header or analysis panel.
- [x] 3.3 Render Summary view using the existing trend summary.
- [x] 3.4 Render Compare view with reference date, loaded count, baseline, historical periods, deltas, and source labels.
- [x] 3.5 Render Compare loading and error states in the analysis panel.
- [x] 3.6 Render Advice view with goal, data status, trend, trend class, interpretation, and recommendation details when present.
- [x] 3.7 Render Advice insufficient-data, loading, and error states in the analysis panel.
- [x] 3.8 Update bottom help text to include analysis view navigation and advice goal controls.

## 4. Tests And Verification

- [x] 4.1 Add tests for analysis view key mappings.
- [x] 4.2 Add state transition tests for switching analysis views and blocking those controls during input modes.
- [x] 4.3 Add data-loading tests proving Compare and Advice call the existing use cases through fake storage.
- [x] 4.4 Add tests that record writes invalidate loaded Compare and Advice states.
- [x] 4.5 Add render tests for the narrower records panel and wider analysis panel.
- [x] 4.6 Add render tests for Compare loaded, loading, and error states.
- [x] 4.7 Add render tests for Advice loaded, insufficient-data, loading, and error states.
- [x] 4.8 Run `cargo fmt`.
- [x] 4.9 Run `cargo test`.
- [x] 4.10 Run `cargo check`.

## 5. Tab Analysis Switching

- [x] 5.1 Update the TUI spec so Tab is the analysis-view switching key in normal mode.
- [x] 5.2 Map Tab to analysis-view rotation instead of bracket keys.
- [x] 5.3 Keep Tab as field switching while adding or editing a record.
- [x] 5.4 Update help text and tests for Tab-based analysis switching.
- [x] 5.5 Run `cargo fmt`.
- [x] 5.6 Run `cargo test`.
- [x] 5.7 Run `cargo check`.
