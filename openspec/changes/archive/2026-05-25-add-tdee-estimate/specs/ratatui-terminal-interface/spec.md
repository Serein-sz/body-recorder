## ADDED Requirements

### Requirement: TUI renders TDEE estimate in Summary analysis
The TUI SHALL render TDEE estimate data in the Summary analysis view using the same calculation behavior as the CLI TDEE estimate.

#### Scenario: Summary view is opened
- **WHEN** the user opens or returns to the Summary analysis view
- **THEN** the TUI loads TDEE estimate data through the TDEE use case when it is not already current

#### Scenario: TDEE estimate is available
- **WHEN** TDEE estimate data is loaded successfully with at least one recent weight record
- **THEN** the TUI displays the estimated kcal/day value in the Summary view when available
- **THEN** the TUI displays the reference date, 7-day average weight, sample count, age, sex, height, activity factor, and data-quality status

#### Scenario: TDEE data is low-sample
- **WHEN** TDEE estimate data is loaded with one or two recent weight records
- **THEN** the TUI displays the estimate with a low-sample status instead of presenting it as normal

#### Scenario: TDEE data is unavailable
- **WHEN** TDEE estimate data is loaded with no recent weight records
- **THEN** the TUI displays that no recent weight data is available
- **THEN** the TUI does not display a kcal/day estimate

#### Scenario: TDEE data cannot be loaded
- **WHEN** TDEE loading fails
- **THEN** the TUI displays the TDEE error in the analysis panel without closing the TUI

### Requirement: TUI refreshes Summary TDEE analysis consistently
The TUI SHALL refresh and invalidate Summary TDEE analysis consistently with other Summary data.

#### Scenario: Summary view refreshes
- **WHEN** the user refreshes while Summary is active
- **THEN** the TUI refreshes recent records
- **THEN** the TUI refreshes TDEE estimate data

#### Scenario: Record data changes
- **WHEN** the user adds, edits, or deletes a weight record
- **THEN** previously loaded TDEE analysis is invalidated or reloaded before being shown as current
