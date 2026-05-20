## ADDED Requirements

### Requirement: Compare output includes BMI context

The system SHALL include BMI values and BMI category labels for available baseline and historical comparison average weights.

#### Scenario: Baseline average is available

- **WHEN** the compare baseline average has a weight value
- **THEN** the compare output includes the calculated BMI and BMI category for the baseline average

#### Scenario: Historical average is available

- **WHEN** a historical comparison point has a direct or filled average weight value
- **THEN** the compare output includes the calculated BMI and BMI category for that average

#### Scenario: Historical average is missing

- **WHEN** a historical comparison point has no average weight value
- **THEN** the compare output displays BMI as unavailable and does not display a BMI category

### Requirement: Compare output colors BMI category labels

The system SHALL color BMI category labels in compare output according to the shared BMI category status colors.

#### Scenario: Compare renders a BMI category

- **WHEN** compare output displays a BMI category label
- **THEN** the category label uses the color required by the BMI category standard
