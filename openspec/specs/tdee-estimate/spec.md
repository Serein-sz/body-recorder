## Purpose

Define behavior for estimating total daily energy expenditure from recent body weight records.

## Requirements

### Requirement: User can estimate TDEE from recent weight records
The system SHALL provide a CLI command that estimates total daily energy expenditure in kcal/day from recent stored weight records.

#### Scenario: TDEE estimate requested with enough recent records
- **WHEN** the user requests a TDEE estimate and at least three weight records exist in the latest 7-day window
- **THEN** the system prints an estimated TDEE in kcal/day
- **THEN** the output includes the 7-day average weight, sample count, age, sex, height, activity factor, and reference date used for the estimate

#### Scenario: TDEE estimate requested with a reference date
- **WHEN** the user requests a TDEE estimate with a reference date
- **THEN** the system estimates TDEE using the 7-day window ending on that reference date

### Requirement: TDEE uses the configured personal baseline assumptions
The system SHALL calculate the initial TDEE estimate using the male Mifflin-St Jeor BMR formula with the fixed profile assumptions for this application.

#### Scenario: Calculation basis is available
- **WHEN** recent weight data supports a TDEE estimate
- **THEN** the system calculates age from birth date 2001-03-06 and the reference date
- **THEN** the system uses male sex, height 173 cm, and activity factor 1.60
- **THEN** the system calculates TDEE as BMR multiplied by the activity factor

### Requirement: TDEE estimate reports sparse data cautiously
The system SHALL distinguish normal, low-sample, and unavailable TDEE estimate states.

#### Scenario: One or two recent records exist
- **WHEN** the user requests a TDEE estimate and one or two weight records exist in the latest 7-day window
- **THEN** the system prints an estimated TDEE in kcal/day
- **THEN** the output marks the estimate as low-sample and includes the sample count

#### Scenario: No recent records exist
- **WHEN** the user requests a TDEE estimate and no weight records exist in the latest 7-day window
- **THEN** the system reports that no recent weight data is available
- **THEN** the system does not print a kcal/day estimate

### Requirement: TDEE output remains non-medical and estimate-oriented
The system SHALL present TDEE as a practical estimate rather than medical advice or a precise prescription.

#### Scenario: Estimate is printed
- **WHEN** the system prints a TDEE estimate
- **THEN** the output labels it as an estimate
- **THEN** the output avoids diagnosis, meal plans, or claims of exact accuracy
