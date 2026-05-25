## Context

The application currently records body weight, calculates BMI, compares historical averages, provides trend-based diet advice, and renders those views in both CLI and Ratatui TUI surfaces. It does not expose a kcal/day energy baseline, so users must interpret diet intake and trend advice outside the tool.

TDEE estimation can be derived without a new database table by combining recent weight records with fixed personal profile assumptions. The existing repository already supports date-bounded weight queries, and the app/use-case layer is the right place to load the relevant records before delegating pure calculations to the domain layer.

## Goals / Non-Goals

**Goals:**

- Estimate TDEE in kcal/day from the latest 7-day average body weight.
- Use fixed initial profile assumptions: male, born 2001-03-06, height 173 cm, activity factor 1.60.
- Provide a CLI command for the estimate and calculation basis.
- Display the estimate in the TUI with clear data-quality status.
- Keep calculation logic deterministic and unit-testable.

**Non-Goals:**

- Storing TDEE estimates or personal profile settings in Supabase.
- Tracking daily calorie intake, exercise calories, or adaptive TDEE from intake logs.
- Replacing existing trend-based diet advice with exact calorie prescriptions.
- Providing medical or diagnostic recommendations.

## Decisions

1. **Derive TDEE from recent stored weights instead of asking for weight input.**
   The tool already owns weight records, and the user requested using the latest week average from the interface. The use case will fetch records from `reference_date - 6 days` through `reference_date` and average available records in that range.

   Alternative considered: accept `--weight-kg` on the CLI. This is simpler for isolated calculation but duplicates data the tool already stores and can drift from the user's recorded trend.

2. **Use Mifflin-St Jeor BMR with a fixed activity factor.**
   The estimate will calculate age from the reference date and birth date, compute male BMR from weight, height, and age, then multiply by `1.60`. This matches the user's current profile and training pattern as an initial baseline.

   Alternative considered: separate training-day and rest-day estimates. This may be useful later, but a single weekly baseline is easier to validate and display first.

3. **Return structured data-quality status from the domain layer.**
   A TDEE result should distinguish no data, low sample count, and normal estimate. Three or more records in the 7-day window count as normal; one or two records can produce an estimate marked low-sample; zero records cannot produce an estimate.

   Alternative considered: treat fewer than three records as an error. That would be stricter, but it would block useful estimates when the user has sparse recent data.

4. **Integrate TDEE into the Summary analysis surface.**
   CLI gets a `tdee` command. TUI should expose TDEE inside the Summary analysis view so the user's first analysis screen shows weight trend, BMI context, and daily energy baseline together.

   Alternative considered: add a separate TDEE analysis view. This gives TDEE its own loading state and more space, but it adds another tab for information that belongs with the user's high-level summary.

## Risks / Trade-offs

- Fixed profile assumptions can become stale if height, activity, or birthday assumptions need to change -> Keep constants centralized so a later profile/config change can replace them cleanly.
- Formula-based TDEE can be inaccurate for an individual -> Label output as an estimate and include basis and sample count.
- Sparse recent records can make the estimate jumpy -> Mark one or two records as low-sample and show the sample count.
- Summary becomes denser with TDEE included -> Keep the TDEE display concise while still showing estimate, sample count, status, and basis.
