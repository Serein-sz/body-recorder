## ADDED Requirements

### Requirement: TUI add form supports arrow date selection

The TUI SHALL allow users to select the add-record date with left and right arrow keys while the add-record date field is active.

#### Scenario: Blank add date moves to previous day

- **WHEN** the user is adding a record
- **AND** the date field is active
- **AND** the date field is blank
- **AND** the user presses the left arrow key
- **THEN** the add form date is set to one day before the TUI reference date in `YYYY-MM-DD` format
- **THEN** no storage write occurs

#### Scenario: Blank add date moves to next day

- **WHEN** the user is adding a record
- **AND** the date field is active
- **AND** the date field is blank
- **AND** the user presses the right arrow key
- **THEN** the add form date is set to one day after the TUI reference date in `YYYY-MM-DD` format
- **THEN** no storage write occurs

#### Scenario: Existing add date moves by one day

- **WHEN** the user is adding a record
- **AND** the date field is active
- **AND** the date field contains a valid date
- **AND** the user presses the left or right arrow key
- **THEN** the add form date is adjusted by one day in the corresponding direction
- **THEN** no storage write occurs

#### Scenario: Invalid typed add date is not changed by arrow selection

- **WHEN** the user is adding a record
- **AND** the date field is active
- **AND** the date field contains an invalid date
- **AND** the user presses the left or right arrow key
- **THEN** the add form date remains unchanged
- **THEN** the TUI remains under user control

#### Scenario: Arrow date selection is scoped to the add date field

- **WHEN** the user is not adding a record with the date field active
- **AND** the user presses the left or right arrow key
- **THEN** the TUI does not modify the add-record date
