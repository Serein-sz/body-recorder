## Context

The TUI currently renders recent records as text rows and renders Summary, Compare, Advice, and Target analysis as text. Ratatui is already a dependency, and its built-in `Chart` widget can add visual trend context without adding new crates or changing storage.

The most useful chart signal is body weight over time. Other values currently shown in the TUI are either categorical advice, sparse comparison points, or single estimates where charts could imply more precision than the data supports.

## Goals / Non-Goals

**Goals:**

- Add a readable weight trend chart to the Summary analysis view when enough records and terminal space are available.
- Keep chart rendering derived from already loaded recent records.
- Keep the Recent records panel text-only for scanning and editing records.
- Preserve existing keyboard workflows and textual analysis details.
- Avoid charts in panels where they do not improve comprehension.

**Non-Goals:**

- Adding external charting dependencies.
- Adding charts to Recent records, Advice, Target, TDEE, or Compare in this change.
- Fetching additional historical data solely for chart rendering.
- Replacing textual values, labels, or accessibility-friendly state messages with charts.

## Decisions

1. **Use Ratatui built-in chart widgets.**
   Summary will use `Chart` or a chart-like Ratatui rendering with axes/labels where the available area is large enough.

   Alternative considered: custom Unicode sparklines everywhere. This is lightweight, but Ratatui widgets handle layout and styling better in terminal buffers.

2. **Use existing recent records as the chart data source.**
   Charts should be generated from `app.records`, sorted oldest to newest, and should not trigger extra storage calls. This keeps the feature presentational and avoids changing repository behavior.

   Alternative considered: fetch a dedicated 28-day range for charts. That could give a more consistent time window, but it expands the feature into data-loading behavior and can diverge from the visible record list.

3. **Add charts only where they fit the panel purpose.**
   Summary is trend-oriented and benefits from a visual shape. Recent records is an editing/scanning list, Advice is explanatory, TDEE is a single estimate, Target is a projection, and Compare is already a dense table; those remain text-first for now.

   Alternative considered: add a sparkline to Recent records, bars to Compare, and a gauge to Target. These are plausible future improvements, but they risk making the first chart pass noisy.

4. **Gracefully degrade to text-only.**
   If there are fewer than two records, missing space, or a very small terminal, the TUI should skip the chart and keep existing text visible.

## Risks / Trade-offs

- Charts can crowd the terminal on small screens -> Use minimum area checks and keep text as the fallback.
- Records are listed newest first while charts read oldest to newest -> Normalize chart data order before rendering.
- Outlier weights can flatten the chart visually -> Scale chart bounds from the min/max of visible records with a small padding.
- Snapshot-style TUI tests can become brittle -> Test for chart presence and key labels rather than exact buffer art where possible.
