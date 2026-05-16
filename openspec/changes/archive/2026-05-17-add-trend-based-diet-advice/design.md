## Context

The CLI currently records daily body weight and compares a recent four-week average with earlier periods. The new advice command should reuse the same stored weight records but answer a different question: whether recent trend data supports a diet adjustment for a stated goal.

Daily body weight is noisy, so the design must avoid interpreting single-day changes as meaningful. The command also needs to stay within the scope of a personal tracking CLI: it can suggest direction and intensity, but it must not present medical advice, meal plans, or precise nutrition prescriptions.

## Goals / Non-Goals

**Goals:**

- Analyze recent weight data using smoothed windows rather than single-day values.
- Classify the medium-term weight trend in kilograms per week.
- Interpret the trend against a goal: fat loss, maintenance, or weight gain.
- Produce concise diet guidance with direction, intensity, rationale, and data confidence.
- Refuse or soften advice when the data is too sparse or too noisy.
- Keep behavior deterministic and testable.

**Non-Goals:**

- Track meals, calories, macros, activity, body composition, or health conditions.
- Generate medical, diagnostic, or therapeutic advice.
- Persist long-term goal settings in the MVP.
- Add a new database table or Supabase schema migration.
- Use an LLM or external service to generate advice.

## Decisions

1. Use an optional CLI goal argument for the MVP.

   The advice command will accept a goal such as `cut`, `maintain`, or `gain`, and default to `cut` when the goal is omitted. This keeps the common fat-loss path concise while avoiding config persistence before the advice model is proven useful.

   Alternative considered: require the goal every time. That is explicit, but it adds friction for the expected default use case. Storing a default goal in local config was also considered, but it adds configuration behavior and update semantics that are not necessary for the first implementation.

2. Base advice on medium-term trend, with short-term context.

   The command will fetch enough recent records to compute a 28-day trend and a 7-day recent average. The medium-term trend drives the recommendation; the short-term value is supporting context for explaining recent movement.

   Alternative considered: compare only the first and last records. That is simpler, but too sensitive to noisy endpoints.

3. Require enough samples before giving diet adjustment advice.

   A trend result will include data sufficiency. If there are too few records in the analysis window, the command will report that the data is insufficient and avoid recommending a diet adjustment.

   Alternative considered: always produce advice with a low-confidence label. That creates a risk of users acting on weak data, so the MVP should withhold adjustment guidance when the signal is poor.

4. Implement a deterministic rules engine.

   Trend classes and goal-specific advice will be mapped through static rules. This keeps the feature transparent, fast, offline after data retrieval, and easy to cover with unit tests.

   Alternative considered: generate free-form advice with an AI model. That would create dependency, cost, and repeatability concerns, and would be harder to validate.

5. Express advice as direction and intensity, not exact calories.

   Output should use categories like `keep current diet`, `slightly reduce intake`, or `slightly increase intake`, with practical low-risk actions. Exact calorie targets require user attributes and dietary context the app does not collect.

   Alternative considered: prescribe calorie deltas. That can appear more precise, but would be false precision with the current data model.

## Risks / Trade-offs

- Sparse records produce misleading trends -> require minimum sample counts and display confidence.
- Water retention or abnormal weeks distort the signal -> prefer 28-day trend and include cautious language.
- Users may treat CLI output as medical advice -> include concise wording that the advice is trend-based and non-medical.
- Goal semantics can be too coarse -> keep the initial goal set small and expand only after the command proves useful.
- Defaulting the goal to fat loss may not match every user -> make alternate goals explicit with `maintain` and `gain`.
