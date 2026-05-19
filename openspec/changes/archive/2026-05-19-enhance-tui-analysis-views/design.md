## Context

The current TUI has a stable split layout: recent records on the left, a compact trend panel on the right, and a status/help panel at the bottom. The `compare` and `advice` commands already exist as use cases and domain models, but their richer output is only available in non-interactive CLI form.

The analysis content is wider than the records list. Compare needs a table with period, average, delta, and source. Advice needs multiple summary and recommendation lines. The current 62/38 records/trend split gives too much space to records and too little to analysis.

## Goals / Non-Goals

**Goals:**

- Add Summary, Compare, and Advice analysis views inside `br tui`.
- Keep the records list visible while the right panel switches between analysis views.
- Reduce the records panel width so analysis views can render useful content.
- Reuse `use_cases::compare` and `use_cases::advice` for analysis loading.
- Keep record add/edit/delete workflows available and scoped to the records list.
- Model compare/advice loading and errors independently from the record-list status.
- Support changing the advice goal from the TUI.

**Non-Goals:**

- Replace existing `compare` or `advice` CLI command output.
- Add charts or full-screen detail views in the initial version.
- Add mouse interaction.
- Change the Supabase data model or repository trait.

## Decisions

### Use a right-side analysis view selector

The TUI will keep records visible on the left and use the right panel for `Summary`, `Compare`, and `Advice` views. This preserves record context while allowing the user to inspect analyses without leaving the TUI.

Alternative considered: create separate full-screen pages for compare and advice. That gives more space, but it makes the app feel mode-heavy and loses the always-visible record context. Full-screen detail can be added later if the right panel becomes too constrained.

### Make the analysis panel wider than records

The layout will reduce the records list width and give the analysis panel the larger share. A fixed 40/60 split is sufficient for the first implementation and directly addresses the compare/advice width need.

Alternative considered: implement responsive breakpoints immediately. That may become useful for narrow terminals, but it adds layout complexity before the first analysis views exist.

### Use lazy loading per analysis view

Compare and Advice will each have their own load state. The TUI can load an analysis when the user switches to that view or refreshes it. This keeps startup fast and reuses the existing use-case functions.

Alternative considered: load one year of records once and derive all views locally. That could reduce network calls, but it would bypass the current compare/advice use-case orchestration and require more state synchronization after writes.

### Keep record refresh and analysis refresh distinct

Refreshing records will reload the recent records list. Refreshing a compare/advice view will reload that active analysis. After record writes, the records list should refresh and stale analysis views should be invalidated or reloaded before display.

Alternative considered: always refresh all data after every write. That is simpler to reason about, but it performs unnecessary compare/advice requests when the user is only managing records.

### Represent advice goal as TUI state

Advice will default to fat loss, matching the CLI behavior. The TUI will support cycling or selecting fat loss, maintenance, and weight gain goals, then reload advice for the selected goal.

Alternative considered: only show the default advice goal. That would underuse the existing advice capability and force users back to the CLI for maintenance or gain goals.

## Risks / Trade-offs

- Right panel may still be cramped on narrow terminals -> Keep content concise and preserve room for future responsive or full-screen detail behavior.
- Lazy-loaded analyses can become stale after record writes -> Invalidate compare/advice load states after add/edit/delete and reload on demand.
- More TUI state increases complexity -> Introduce small enums for active view and analysis load state instead of folding everything into global status.
- Advice goal interaction can conflict with text input modes -> Only allow analysis view/goal controls in normal mode.
