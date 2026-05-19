## Context

The TUI currently uses rounded panels and analysis views, but most text uses the terminal default foreground. The only strong styling cue is the selected record row using cyan and bold. With records, status, compare values, advice values, loading states, and error states sharing a similar visual weight, the interface is harder to scan than it needs to be.

Terminal color support and user themes vary, so the color system must be conservative and semantic. The interface must keep textual labels for meaning and use color as a readability layer, not as the only source of information.

## Goals / Non-Goals

**Goals:**

- Add a small semantic styling layer for TUI rendering.
- Make panels, titles, active tabs, selected rows, statuses, compare values, and advice signals easier to scan.
- Preserve textual labels for color-coded states and data values.
- Keep colors low-contrast enough that borders and labels do not overpower content.
- Add render tests that check important style intent without snapshotting entire screens.

**Non-Goals:**

- Add user-configurable themes.
- Add new dependencies.
- Change terminal layout, workflows, keyboard controls, or domain calculations.
- Make color the only way to distinguish status, source, delta, or recommendation meaning.

## Decisions

### Use semantic helper functions instead of inline styles

TUI rendering will use named helpers for panel borders, panel titles, active tabs, selected rows, statuses, compare sources/deltas, and advice statuses. This keeps color decisions consistent and makes later palette changes local.

Alternative considered: style each widget inline. That is quick for a few lines, but the UI now has enough states that inline color choices would drift.

### Keep the palette restrained

The default palette will use cyan for focus/current UI, green for success or favorable values, yellow for loading/caution/filled values, red for errors or unfavorable values, and dark gray for borders or unavailable values.

Alternative considered: assign a separate color to every panel or view. That would make the UI busier and less semantic.

### Preserve text labels for all styled meaning

Compare source values still render as `direct`, `filled`, and `missing`. Status lines still include words such as `error`, `loading`, and `no data`. Advice still renders data status and recommendation text.

Alternative considered: replace some values with color-only markers. That would reduce text but weaken accessibility and terminal-theme resilience.

## Risks / Trade-offs

- Terminal themes may make some colors too subtle -> Prefer standard Ratatui colors and retain labels so the UI remains usable without color.
- Tests for exact styling can become brittle -> Test key spans/styles or rendered buffer cells that represent stable semantic intent.
- More style helpers can add indirection -> Keep helpers small and colocated with TUI rendering unless they grow enough to justify a separate module.
