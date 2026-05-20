## ADDED Requirements

### Requirement: Advice output includes BMI context

The system SHALL include BMI context for the short-term average weight in diet advice output when the average is available.

#### Scenario: Advice short-term average is available

- **WHEN** the advice analysis includes a usable short-term average weight
- **THEN** the advice output includes the calculated BMI and BMI category for that average

#### Scenario: Advice short-term average is unavailable

- **WHEN** the advice analysis does not include a usable short-term average weight
- **THEN** the advice output does not calculate or display a BMI category for that missing average

### Requirement: BMI does not drive diet advice recommendations

The system SHALL keep diet adjustment recommendations based on existing trend and goal logic rather than BMI category.

#### Scenario: BMI category is available during advice

- **WHEN** the system renders diet advice with a BMI category
- **THEN** the recommendation direction and intensity remain determined by the trend analysis and selected goal
