## ADDED Requirements

### Requirement: System calculates BMI from fixed height

The system SHALL calculate body mass index from a weight in kilograms using a fixed height of 1.73 meters.

#### Scenario: BMI is calculated for a weight value

- **WHEN** the system has a weight value in kilograms
- **THEN** it calculates BMI as `weight_kg / (1.73 * 1.73)`
- **THEN** it rounds displayed BMI values to two decimal places

### Requirement: System classifies adult BMI values

The system SHALL classify calculated BMI values using adult BMI categories.

#### Scenario: BMI is underweight

- **WHEN** the calculated BMI is less than 18.5
- **THEN** the system classifies it as `underweight`

#### Scenario: BMI is normal

- **WHEN** the calculated BMI is at least 18.5 and less than 25.0
- **THEN** the system classifies it as `normal`

#### Scenario: BMI is overweight

- **WHEN** the calculated BMI is at least 25.0 and less than 30.0
- **THEN** the system classifies it as `overweight`

#### Scenario: BMI is obesity

- **WHEN** the calculated BMI is at least 30.0
- **THEN** the system classifies it as `obesity`

### Requirement: BMI category labels use status colors

The system SHALL render BMI category labels with status colors on color-capable output surfaces.

#### Scenario: Normal BMI category is rendered

- **WHEN** a BMI category is `normal`
- **THEN** the system renders the category label in green

#### Scenario: Underweight BMI category is rendered

- **WHEN** a BMI category is `underweight`
- **THEN** the system renders the category label in yellow

#### Scenario: Overweight BMI category is rendered

- **WHEN** a BMI category is `overweight`
- **THEN** the system renders the category label in yellow

#### Scenario: Obesity BMI category is rendered

- **WHEN** a BMI category is `obesity`
- **THEN** the system renders the category label in red

### Requirement: BMI display handles missing source values

The system SHALL avoid inventing BMI values when the source weight or average is unavailable.

#### Scenario: Source weight is unavailable

- **WHEN** a displayed weight or average has no value
- **THEN** the system displays BMI as unavailable rather than calculating a category
