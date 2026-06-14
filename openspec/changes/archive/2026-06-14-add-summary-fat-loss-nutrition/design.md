## Context

The TUI Summary analysis currently combines recent trend context, 7-day average BMI context, an optional weight chart, and TDEE estimate data. The requested nutrition targets use the same underlying body-weight records and belong in Summary because they are a high-level, daily planning aid rather than a separate record-management workflow.

The project already keeps calculation rules in `src/domain/stats.rs` and terminal presentation in `src/presentation/tui/ui.rs`. This change should preserve that split: nutrition target math should be reusable and testable without Ratatui, while the Summary panel decides how to render the resulting values.

## Goals / Non-Goals

**Goals:**

- Calculate fat-loss carbohydrate, protein, and fat targets from body weight.
- Support the provided weekly training-duration table, with 6-7 hours per week as the current Summary default.
- Automatically choose a weight basis from existing records: recent 7-day average first, latest record second.
- Render concise daily gram targets in the Summary analysis panel.
- Keep labels estimate-oriented and non-medical.

**Non-Goals:**

- Add editable nutrition settings or a new configuration file.
- Add a CLI nutrition command.
- Store nutrition targets in Supabase.
- Generate meal plans, medical advice, or precise calorie prescriptions.
- Change existing TDEE, BMI, trend, advice, or record CRUD behavior.

## Decisions

1. Put nutrition target calculation in the domain layer.

   The macro table and multiplication by body weight are pure domain behavior. Keeping them near existing trend, BMI, and TDEE calculations allows unit tests to cover the table and rounding without Ratatui rendering concerns.

   Alternative considered: calculate directly in `ui.rs`. That would be faster to wire, but it would make the UI responsible for business rules and make future CLI/config use harder.

2. Model training duration as a small enum or equivalent typed band.

   The table has four known rows: 2-3h, 4-5h, 6-7h, and 8-9h. A typed representation avoids string matching at call sites and lets tests verify each row explicitly.

   Alternative considered: store the table only as display strings. That is less code, but it increases the chance of mixing labels and calculation values.

3. Use 6-7h per week as the Summary default.

   The user stated their training duration is about 6-7 hours, and the app already uses fixed personal assumptions for TDEE. A fixed default keeps this change narrow and consistent with the current personal-profile style.

   Alternative considered: add controls to switch the training band in the TUI. That may become useful later, but it adds state, keyboard help, and persistence questions that are outside this request.

4. Use the 7-day average weight when available, then fall back to the latest record.

   Summary already computes recent 7-day average context, and a short average smooths day-to-day weight noise. When there are not enough recent records for an average but at least one record exists, latest weight still lets the user see useful targets.

   Alternative considered: require a 7-day average. That is more conservative but leaves the panel blank for users who have just started recording.

5. Render daily gram targets only.

   The source table is in grams per kilogram for macronutrients, so Summary should display carbohydrate, protein, and fat grams per day. It should not derive an exact macro-energy target in the first version, because the feature is intended as practical nutrient guidance rather than a precise calorie prescription.

## Risks / Trade-offs

- Fixed 6-7h training band may become stale if the user's training changes -> Keep the calculation table typed so a future configurable band can reuse the same domain code.
- Latest-record fallback can be noisier than an average -> Label the weight basis so the user can tell whether targets came from a recent average or latest record.
- Summary panel may become dense -> Keep the nutrition section to a short heading, basis line, and one target line.
- Users may overinterpret targets as medical advice -> Include estimate-oriented wording and avoid diagnoses, meal plans, or exact calorie prescriptions.
