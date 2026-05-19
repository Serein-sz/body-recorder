## ADDED Requirements

### Requirement: Compare loads one year of records

The system SHALL load records for the `compare` command from the reference date back through one year earlier, inclusive.

#### Scenario: Compare requested with explicit reference date

- **WHEN** the user runs `compare` with a reference date
- **THEN** the system requests records from `reference date - 365 days` through the reference date

#### Scenario: Compare requested without explicit reference date

- **WHEN** the user runs `compare` without a reference date
- **THEN** the system uses the current date as the reference date and requests records from one year earlier through that date

### Requirement: Compare uses direct records before smoothed fills

The system SHALL calculate each historical comparison point from direct records in its target window when direct records exist.

#### Scenario: Direct records exist for a comparison point

- **WHEN** the target window for a comparison point contains one or more records
- **THEN** the system reports the average of those records
- **THEN** the system marks the value as direct

#### Scenario: Direct records do not exist but surrounding records exist

- **WHEN** the target window for a comparison point contains no records
- **AND** the loaded one-year records contain a nearest record before the target date and a nearest record after the target date
- **THEN** the system reports a smoothed filled value interpolated from the surrounding records
- **THEN** the system marks the value as filled

#### Scenario: Comparison point cannot be filled

- **WHEN** the target window for a comparison point contains no records
- **AND** the loaded one-year records do not contain records on both sides of the target date
- **THEN** the system reports the comparison point as missing

### Requirement: Compare keeps baseline conservative

The system SHALL use only direct records from the recent baseline window for the baseline average.

#### Scenario: Recent baseline has direct records

- **WHEN** the recent baseline window contains records
- **THEN** the system reports the direct average for the baseline

#### Scenario: Recent baseline has no direct records

- **WHEN** the recent baseline window contains no records
- **THEN** the system reports the baseline as missing
- **THEN** the system does not calculate deltas from smoothed historical points

### Requirement: Compare output identifies value source

The system SHALL make direct, filled, and missing comparison values distinguishable in the `compare` output.

#### Scenario: Filled value is rendered

- **WHEN** a historical comparison point uses a smoothed filled value
- **THEN** the output identifies the value as filled rather than direct

#### Scenario: Missing value is rendered

- **WHEN** a historical comparison point cannot be calculated or filled
- **THEN** the output identifies the value as missing
