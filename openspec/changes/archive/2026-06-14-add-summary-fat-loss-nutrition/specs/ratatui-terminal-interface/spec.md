## ADDED Requirements

### Requirement: TUI renders fat-loss nutrition targets in Summary analysis
The TUI SHALL render fat-loss nutrition target data in the Summary analysis view using the shared fat-loss nutrition target calculation.

#### Scenario: Summary nutrition targets use recent average weight
- **WHEN** the Summary analysis view has a usable recent 7-day average weight
- **THEN** it displays fat-loss carbohydrate, protein, and fat gram targets calculated from that average
- **THEN** it labels the weight basis as the recent 7-day average
- **THEN** it labels the weekly training-duration band as 6-7h

#### Scenario: Summary nutrition targets use latest weight fallback
- **WHEN** the Summary analysis view does not have a usable recent 7-day average weight
- **AND** at least one weight record is available
- **THEN** it displays fat-loss carbohydrate, protein, and fat gram targets calculated from the latest record weight
- **THEN** it labels the weight basis as latest weight

#### Scenario: Summary nutrition targets are unavailable
- **WHEN** the Summary analysis view has no usable recent 7-day average weight and no weight records
- **THEN** it displays that fat-loss nutrition targets are unavailable
- **THEN** it does not calculate macronutrient targets from missing weight data

#### Scenario: Summary keeps existing context visible
- **WHEN** the Summary analysis view displays fat-loss nutrition targets
- **THEN** the existing trend, BMI, TDEE, and eligible chart context remain visible according to their current behavior
