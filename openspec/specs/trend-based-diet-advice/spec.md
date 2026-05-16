## Purpose

Provide goal-aware, trend-based diet adjustment guidance from recent body weight records.

## Requirements

### Requirement: User can request trend-based diet advice

The system SHALL provide a CLI command that analyzes recent weight records and prints diet adjustment advice for a selected goal, defaulting to fat loss when no goal is provided.

#### Scenario: Advice requested with fat loss goal

- **WHEN** the user requests diet advice with the fat loss goal
- **THEN** the system prints a trend summary, goal interpretation, and diet adjustment recommendation

#### Scenario: Advice requested without goal

- **WHEN** the user requests diet advice without a goal
- **THEN** the system analyzes the trend using the fat loss goal

#### Scenario: Advice requested with reference date

- **WHEN** the user requests diet advice with a reference date
- **THEN** the system analyzes records relative to that date instead of the current date

### Requirement: Advice uses recent trend analysis

The system SHALL base diet advice on smoothed recent weight trends rather than a single recorded weight.

#### Scenario: Medium-term trend is available

- **WHEN** enough records exist in the analysis range
- **THEN** the system reports the medium-term trend as kilograms per week

#### Scenario: Short-term context is available

- **WHEN** enough records exist in the short-term window
- **THEN** the system reports short-term context separately from the medium-term recommendation driver

### Requirement: Advice is goal-aware

The system SHALL interpret the same weight trend differently for fat loss, maintenance, and weight gain goals.

#### Scenario: Trend conflicts with fat loss goal

- **WHEN** the goal is fat loss and the medium-term trend shows meaningful weight gain
- **THEN** the system recommends a conservative intake reduction

#### Scenario: Trend supports fat loss goal

- **WHEN** the goal is fat loss and the medium-term trend shows moderate weight loss
- **THEN** the system recommends keeping the current diet direction

#### Scenario: Trend conflicts with maintenance goal

- **WHEN** the goal is maintenance and the medium-term trend shows meaningful gain or loss
- **THEN** the system recommends a conservative adjustment toward stability

#### Scenario: Trend conflicts with weight gain goal

- **WHEN** the goal is weight gain and the medium-term trend shows meaningful weight loss
- **THEN** the system recommends a conservative intake increase

### Requirement: Advice handles insufficient data cautiously

The system SHALL avoid diet adjustment recommendations when recent records are insufficient to support a trend judgment.

#### Scenario: Too few recent records

- **WHEN** the analysis range contains fewer records than the minimum required sample count
- **THEN** the system reports insufficient data and does not recommend a diet adjustment

#### Scenario: No records in range

- **WHEN** no weight records exist in the analysis range
- **THEN** the system reports that no usable recent records were found

### Requirement: Advice output stays conservative

The system SHALL express advice as trend-based direction and intensity, not as medical advice, diagnosis, meal plans, or exact calorie prescriptions.

#### Scenario: Recommendation is printed

- **WHEN** the system prints a diet adjustment recommendation
- **THEN** the recommendation uses non-medical language and avoids exact calorie targets

#### Scenario: Fast weight change detected

- **WHEN** the medium-term trend shows unusually fast loss or gain
- **THEN** the system flags the change as high intensity and recommends caution rather than escalating aggressive diet changes
