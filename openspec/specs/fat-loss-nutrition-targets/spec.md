## Purpose

Define behavior for calculating practical fat-loss macronutrient targets from body weight and weekly training duration.

## Requirements

### Requirement: System calculates fat-loss nutrition targets by body weight
The system SHALL calculate daily fat-loss macronutrient targets by multiplying body weight in kilograms by the grams-per-kilogram factors for a weekly training-duration band.

#### Scenario: Targets are calculated for 2-3h weekly training
- **WHEN** the system calculates targets for 70.0 kg and the 2-3h weekly training band
- **THEN** it returns 154g carbohydrate, 98g protein, and 56g fat per day

#### Scenario: Targets are calculated for 4-5h weekly training
- **WHEN** the system calculates targets for 70.0 kg and the 4-5h weekly training band
- **THEN** it returns 175g carbohydrate, 112g protein, and 63g fat per day

#### Scenario: Targets are calculated for 6-7h weekly training
- **WHEN** the system calculates targets for 70.0 kg and the 6-7h weekly training band
- **THEN** it returns 210g carbohydrate, 119g protein, and 70g fat per day

#### Scenario: Targets are calculated for 8-9h weekly training
- **WHEN** the system calculates targets for 70.0 kg and the 8-9h weekly training band
- **THEN** it returns 245g carbohydrate, 126g protein, and 70g fat per day

### Requirement: Fat-loss nutrition targets use the user's current training profile
The system SHALL use the 6-7h weekly training-duration band as the current default fat-loss nutrition profile.

#### Scenario: Default profile is requested
- **WHEN** the system calculates fat-loss nutrition targets without an explicit weekly training-duration band
- **THEN** it uses 3.0g/kg carbohydrate, 1.7g/kg protein, and 1.0g/kg fat

### Requirement: Fat-loss nutrition output remains estimate-oriented
The system SHALL present fat-loss nutrition targets as practical estimates rather than medical advice, meal plans, diagnoses, or precise prescriptions.

#### Scenario: Targets are displayed
- **WHEN** the system displays fat-loss nutrition targets
- **THEN** the output labels them as estimate-oriented targets
- **THEN** the output does not present a meal plan or diagnosis
