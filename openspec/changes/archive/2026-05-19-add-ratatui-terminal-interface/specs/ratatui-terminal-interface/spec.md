## ADDED Requirements

### Requirement: User can launch an interactive terminal interface

The system SHALL provide a `tui` command that launches an interactive terminal interface without changing existing command behavior.

#### Scenario: TUI command is invoked

- **WHEN** a user runs `br tui`
- **THEN** the system enters an interactive terminal interface backed by the configured storage

#### Scenario: Existing commands are invoked

- **WHEN** a user invokes an existing non-TUI command with currently supported arguments
- **THEN** the command remains accepted with the same meaning and output behavior

### Requirement: TUI shows recent weight records

The TUI SHALL display recent weight records loaded through the application storage interface.

#### Scenario: Records are loaded

- **WHEN** the TUI loads recent records successfully
- **THEN** it displays the records with their dates and weights

#### Scenario: No records are available

- **WHEN** the TUI loads an empty record list
- **THEN** it displays an empty-state message instead of failing

#### Scenario: Loading fails

- **WHEN** recent records cannot be loaded
- **THEN** the TUI displays an actionable error state and remains under user control

### Requirement: TUI uses rounded panel borders

The TUI SHALL render its visible panels and modal overlays with rounded terminal borders.

#### Scenario: Main panels are rendered

- **WHEN** the TUI renders the header, records, trend, and status panels
- **THEN** those panels use rounded border corners

#### Scenario: Modal overlay is rendered

- **WHEN** the TUI renders an add, edit, or delete confirmation overlay
- **THEN** the overlay uses rounded border corners

### Requirement: User can refresh TUI data

The TUI SHALL allow the user to refresh displayed weight data from storage.

#### Scenario: Refresh succeeds

- **WHEN** the user triggers refresh
- **THEN** the TUI reloads recent records and updates the displayed list

#### Scenario: Refresh fails

- **WHEN** the user triggers refresh and storage returns an error
- **THEN** the TUI keeps the previous records visible and displays the error

### Requirement: User can manage weight records from the TUI

The TUI SHALL allow the user to add a weight record, edit a selected record, and delete a selected record using existing command behavior.

#### Scenario: Add record succeeds

- **WHEN** the user submits a valid weight and optional date from the TUI
- **THEN** the system saves the record through the same use-case behavior as the add command
- **THEN** the TUI refreshes or updates the displayed records

#### Scenario: Edit record succeeds

- **WHEN** the user selects a record and submits a valid replacement weight
- **THEN** the system updates the record through the same use-case behavior as the update command
- **THEN** the TUI refreshes or updates the displayed records

#### Scenario: Delete record succeeds

- **WHEN** the user selects a record and confirms deletion
- **THEN** the system deletes the record through the same use-case behavior as the delete command
- **THEN** the TUI refreshes or updates the displayed records

#### Scenario: Input is invalid

- **WHEN** the user submits an invalid date or weight from the TUI
- **THEN** the TUI displays the validation error without sending invalid data to storage

### Requirement: TUI presents trend context

The TUI SHALL show lightweight trend or advice context derived from existing domain calculations when enough recent data is available.

#### Scenario: Trend context is available

- **WHEN** recent records support trend or advice calculation
- **THEN** the TUI displays a concise trend or advice summary derived from the existing domain logic

#### Scenario: Trend context is unavailable

- **WHEN** recent records do not support trend or advice calculation
- **THEN** the TUI displays the unavailable or insufficient-data state without making a recommendation

### Requirement: TUI handles terminal lifecycle safely

The TUI SHALL restore terminal state when it exits normally or after an application error.

#### Scenario: User exits the TUI

- **WHEN** the user requests exit from the TUI
- **THEN** the system leaves the alternate screen and disables raw mode before returning to the shell

#### Scenario: TUI operation returns an error

- **WHEN** the TUI exits because an operation returns an unrecoverable error
- **THEN** the system leaves the alternate screen and disables raw mode before reporting the error

### Requirement: TUI remains keyboard operable

The TUI SHALL support the initial workflow with keyboard controls.

#### Scenario: User navigates records

- **WHEN** records are displayed and the user presses supported navigation keys
- **THEN** the TUI updates the selected record without modifying stored data

#### Scenario: User invokes a command key

- **WHEN** the user presses a supported command key such as add, edit, delete, refresh, or quit
- **THEN** the TUI maps the key to the corresponding interface action
