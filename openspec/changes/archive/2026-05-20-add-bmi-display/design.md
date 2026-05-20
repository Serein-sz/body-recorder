## Context

The application already has a central statistics module for weight averages, comparison points, and trend analysis, with separate renderers for plain CLI output and Ratatui UI output. BMI is derived from existing weight values and a fixed personal height, so it can be added without changing storage, repository behavior, command arguments, or Supabase schema.

The user wants BMI shown with the applicable adult standard/category and with color rendering in the right places. The hard-coded height is `1.73m`.

## Goals / Non-Goals

**Goals:**

- Provide a single domain-level BMI calculation using `weight_kg / (1.73 * 1.73)`.
- Classify BMI into adult categories: underweight, normal, overweight, obesity.
- Reuse existing CLI ANSI styling and TUI Ratatui styling to color BMI category labels.
- Add BMI context anywhere the app presents individual weight values or analysis averages where the extra context fits the current layout.
- Keep behavior deterministic and testable with no network dependency.

**Non-Goals:**

- Do not store height or BMI in Supabase.
- Do not add CLI options, config files, or prompts for editing height.
- Do not provide medical diagnosis or change diet advice recommendation logic based on BMI.
- Do not introduce new styling dependencies.

## Decisions

1. Centralize BMI calculation and category classification in `src/stats.rs`.

   Rationale: BMI is domain calculation, not renderer logic. Keeping calculation in `stats` allows CLI, TUI, and tests to share one source of truth.

   Alternative considered: define formatting helpers only in `output.rs` and `tui/ui.rs`. That would duplicate thresholds and make future height changes harder.

2. Use a fixed public constant for height.

   Rationale: The requirement explicitly allows hard-coding the user's height as `1.73m`. A named constant such as `HEIGHT_METERS` makes the choice visible and easy to replace later with configuration.

   Alternative considered: add height to config or command arguments now. That would increase scope and require more UX decisions than the current request needs.

3. Treat BMI category as a typed enum with label helpers.

   Rationale: A `BmiCategory` enum avoids scattering string comparisons through renderers and gives both CLI and TUI a stable input for color selection.

   Alternative considered: return only a string label from classification. That is simpler but weaker for styling and tests.

4. Color only the BMI category label, not the numeric BMI value.

   Rationale: The category carries the status signal; coloring the label is compact and consistent with existing status styling. It avoids making numeric columns harder to scan.

   Alternative considered: color the full BMI cell. That is visually louder and more likely to interfere with table alignment in ANSI output.

5. Add BMI to output surfaces that already present weight values or averages.

   Rationale: BMI should appear in context with weight, not as a separate standalone screen. Relevant surfaces are list records, compare baseline and points, advice trend short-term average, TUI recent records, TUI summary, TUI compare, and TUI advice.

   Alternative considered: show BMI only in summary/advice. That misses the user's request to add it in suitable positions and would leave record-level context absent.

## Risks / Trade-offs

- Fixed height may become stale if the app is used by another person -> keep the height as a visible constant and avoid persisting derived BMI values.
- BMI categories are screening categories, not medical advice -> display category labels only and do not alter diet recommendations or add diagnostic language.
- Extra columns can crowd terminal layouts -> keep labels short, use existing compact formatting, and prefer omitting less important detail over breaking alignment in narrow TUI areas.
- Color behavior differs between ANSI CLI and Ratatui TUI -> implement small renderer-specific style helpers backed by the shared category enum.
