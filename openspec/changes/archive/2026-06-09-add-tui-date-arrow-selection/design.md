## Context

The TUI already routes terminal key events through `Action` values before `App` applies mode-specific behavior. The add-record form stores date and weight in `InputState`, starts with a blank date displayed as `today`, and saves through the existing add use case after validation.

## Goals / Non-Goals

**Goals:**

- Let users move the add-record date backward or forward by one day with arrow keys.
- Preserve the current optional-date behavior: a blank date still means the normal add-command default when the user saves without selecting a date.
- Keep the behavior local to TUI input handling and avoid storage or use-case changes.
- Make the control visible in the TUI help text.

**Non-Goals:**

- Add a calendar picker or month/year jump controls.
- Change edit-form date behavior.
- Change normal-mode record navigation or analysis view switching.
- Change CLI add command behavior.

## Decisions

1. **Represent arrow keys as explicit actions.**
   Add separate actions for previous-date and next-date selection, then map left and right arrow keys in the event layer. This keeps raw terminal events out of `App` and follows the existing action-dispatch pattern.

   Alternative considered: convert left/right directly inside the event loop based on current mode. That would couple event reading to app state, while the current design keeps interpretation centralized in `App`.

2. **Apply date selection only in add mode when the date field is active.**
   The form already has an active field model, so arrow selection should respect focus. This avoids changing dates while the user is entering a weight and keeps edit mode unchanged.

   Alternative considered: allow left/right anywhere in the add form. That is faster for one-field workflows but can surprise users while the weight field is active.

3. **Use the TUI reference date as the base for a blank date.**
   A blank add date is rendered as `today`; using `reference_date` as the first arrow-selection base makes the visual default and state transition match. Pressing left from blank produces yesterday, and pressing right produces tomorrow.

   Alternative considered: initialize add mode with today's date. That simplifies adjustment but changes save semantics by always submitting an explicit date.

4. **Leave invalid typed dates unchanged on arrow selection.**
   If the user manually enters an invalid date, arrow selection should not guess intent. The existing submit-time validation remains the authoritative error path.

   Alternative considered: reset invalid date input to the reference date. That is forgiving but can silently discard user input.

## Risks / Trade-offs

- Invalid manual dates do not recover automatically through arrows -> Keep the input unchanged and rely on existing validation/error feedback.
- Users may expect arrow keys to work from the weight field -> Update help text to mention date-field scope.
- Right arrow can select a future date -> The existing add-date validation and use case define whether that date is acceptable; this change does not add a future-date policy.
