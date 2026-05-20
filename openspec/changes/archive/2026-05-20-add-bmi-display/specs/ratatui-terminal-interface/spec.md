## ADDED Requirements

### Requirement: TUI displays BMI context

The TUI SHALL display BMI values and BMI category labels where it presents individual weights or calculated weight averages.

#### Scenario: Recent records are displayed

- **WHEN** the TUI displays recent weight records
- **THEN** each record includes the calculated BMI and BMI category for that record's weight

#### Scenario: Summary view is displayed

- **WHEN** the TUI displays summary trend context with a usable short-term average
- **THEN** the summary includes the calculated BMI and BMI category for that average

#### Scenario: Compare view is displayed

- **WHEN** the TUI displays compare baseline or historical average values
- **THEN** each available average includes the calculated BMI and BMI category for that average

#### Scenario: Advice view is displayed

- **WHEN** the TUI displays advice analysis with a usable short-term average
- **THEN** the advice analysis includes the calculated BMI and BMI category for that average

### Requirement: TUI colors BMI category labels

The TUI SHALL render BMI category labels with Ratatui styles that match the shared category status colors.

#### Scenario: TUI renders a BMI category

- **WHEN** a BMI category label is visible in the TUI
- **THEN** the category label uses the color required by the BMI category standard
