## Context

`compare` is currently implemented through the existing module boundaries: `use_cases::compare` chooses the date range and loads records through `WeightRepository`, `stats::compare_weights` calculates comparison periods, and `output::render_comparison` formats the result. The current range starts 372 days before the reference date so the one-year comparison point can use a seven-day lookback window, which means the command reads records older than one year.

The comparison rows also depend on direct records in each target window. Sparse tracking can therefore produce `no data` for historical points even when nearby records inside the one-year range could support a cautious estimate.

## Goals / Non-Goals

**Goals:**

- Fetch only records from `reference_date - 365 days` through `reference_date`, inclusive.
- Keep storage access behind `WeightRepository::list_weights_between`.
- Fill empty historical comparison windows with a transparent smoothed value when surrounding records inside the loaded range make that reasonable.
- Preserve explicit missing output when no defensible fill exists.
- Keep CLI arguments and Supabase schema unchanged.

**Non-Goals:**

- Add new persistent fields or change the `weight_records` table.
- Change diet advice trend analysis.
- Produce medical guidance or interpret weight changes beyond the existing lower/steady/higher comparison language.
- Hide whether a value was directly observed or filled.

## Decisions

1. Use a strict one-year inclusive fetch range.

   `comparison_range(reference_date)` will return `(reference_date - 365 days, reference_date)`. This makes the data boundary match user expectations and avoids reading older records solely to populate the one-year row.

   Alternative considered: keep the existing 372-day range and only filter display values. That preserves more direct averages near the one-year point, but it violates the requested one-year retrieval boundary.

2. Fill only target comparison points, not the recent baseline.

   Historical points represent dated comparison anchors and can be estimated from nearby surrounding records. The recent baseline is the current comparison reference; if the recent four-week period has no direct records, deltas remain unavailable because a filled baseline would make every comparison depend on an inferred current weight.

   Alternative considered: fill every empty period, including the baseline. That maximizes table completeness but makes the most important comparison anchor less trustworthy.

3. Prefer direct window averages before smoothed fills.

   Each historical point will first use the existing target-window average. If the target window has no direct records, the domain calculation will look for the closest record on or before the target date and the closest record on or after the target date within the loaded one-year records. When both exist and are different dates, it will interpolate linearly by date between those two weights and mark the point as filled.

   Alternative considered: carry the nearest single record forward or backward. That is simpler, but it can overstate confidence when the target sits far from the nearest record and does not represent smoothing between known values.

4. Add source metadata to comparison values.

   `ComparisonPoint` will carry a value source such as direct, smoothed fill, or missing. Rendering can show the source/status without needing to inspect calculation details. Direct averages continue to display their sample count; filled values show that the direct sample count was zero and the value is inferred.

   Alternative considered: encode source only in the rendered status string. Keeping it in domain output is more testable and avoids coupling calculation behavior to terminal text.

## Risks / Trade-offs

- Smoothed fills can imply precision from sparse data -> mark filled values explicitly and keep missing when both surrounding records are not available.
- The one-year row may be harder to fill because records before the one-year boundary are not loaded -> accept missing output rather than read outside the requested range.
- Interpolation assumes roughly linear movement between two records -> use it only as a display aid for comparison, not as advice or diagnosis.
