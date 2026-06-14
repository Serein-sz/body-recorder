## 1. Domain Nutrition Targets

- [x] 1.1 Add a typed weekly training-duration band model covering 2-3h, 4-5h, 6-7h, and 8-9h.
- [x] 1.2 Add fat-loss macronutrient factor data for each training-duration band.
- [x] 1.3 Implement fat-loss nutrition target calculation from body weight, returning daily carbohydrate, protein, and fat grams.
- [x] 1.4 Add a default fat-loss nutrition profile that uses the 6-7h weekly training-duration band.
- [x] 1.5 Add domain unit tests for all table rows using a 70.0 kg example.

## 2. Summary Weight Basis

- [x] 2.1 Add logic for Summary nutrition to prefer the recent 7-day average weight when available.
- [x] 2.2 Add latest-record fallback when the recent 7-day average weight is unavailable.
- [x] 2.3 Add unavailable handling when no usable weight basis exists.
- [x] 2.4 Test average, fallback, and unavailable basis selection.

## 3. TUI Summary Rendering

- [x] 3.1 Render a concise Fat loss nutrition section in the Summary analysis panel.
- [x] 3.2 Show the weight basis label, basis weight, and 6-7h weekly training-duration label.
- [x] 3.3 Show carbohydrate, protein, and fat daily gram targets.
- [x] 3.4 Show an estimate-oriented note and avoid meal-plan or medical-prescription wording.
- [x] 3.5 Preserve existing Summary trend, BMI, TDEE, and chart behavior.

## 4. Verification

- [x] 4.1 Add or update TUI rendering tests for average-basis nutrition targets.
- [x] 4.2 Add or update TUI rendering tests for latest-weight fallback and unavailable states.
- [x] 4.3 Run `cargo fmt`.
- [x] 4.4 Run `cargo test`.
- [x] 4.5 Run `openspec status --change add-summary-fat-loss-nutrition`.
