## Purpose

Define behavior for the interactive Ratatui terminal interface used to review and manage body weight records.
## Requirements
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

### Requirement: TUI supports analysis views

The TUI SHALL allow users to switch the analysis panel between Summary, Compare, and Advice views while keeping the recent records list visible.

#### Scenario: User switches analysis views

- **WHEN** the user presses Tab while in normal mode
- **THEN** the TUI changes the active analysis view
- **THEN** the recent records list remains visible

#### Scenario: User is editing record input

- **WHEN** the user is adding or editing a record
- **THEN** analysis-view navigation keys do not change the active analysis view

### Requirement: TUI gives analysis more width than records

The TUI SHALL allocate less horizontal space to the records panel than to the analysis panel in the standard split layout.

#### Scenario: Split layout is rendered

- **WHEN** the TUI renders the records and analysis panels side by side
- **THEN** the analysis panel receives more horizontal width than the records panel

### Requirement: TUI renders compare analysis

The TUI SHALL render historical comparison data in the Compare analysis view using existing compare behavior.

#### Scenario: Compare view is opened

- **WHEN** the user switches to the Compare analysis view
- **THEN** the TUI loads comparison data through the existing compare use case

#### Scenario: Compare data is available

- **WHEN** comparison data is loaded successfully
- **THEN** the TUI displays the reference date, loaded record count, recent baseline, historical period averages, deltas, and value sources

#### Scenario: Compare data cannot be loaded

- **WHEN** comparison loading fails
- **THEN** the TUI displays the compare error in the analysis panel without closing the TUI

### Requirement: TUI renders advice analysis

The TUI SHALL render diet advice data in the Advice analysis view using existing advice behavior.

#### Scenario: Advice view is opened

- **WHEN** the user switches to the Advice analysis view
- **THEN** the TUI loads advice data through the existing advice use case using the current advice goal

#### Scenario: Advice data is available

- **WHEN** advice data is loaded successfully
- **THEN** the TUI displays the goal, data status, trend, trend class, interpretation, and recommendation details when present

#### Scenario: Advice data is insufficient

- **WHEN** advice data is loaded but does not support a recommendation
- **THEN** the TUI displays the insufficient or unavailable data state without presenting a diet adjustment recommendation

#### Scenario: Advice data cannot be loaded

- **WHEN** advice loading fails
- **THEN** the TUI displays the advice error in the analysis panel without closing the TUI

### Requirement: User can change TUI advice goal

The TUI SHALL let users change the advice goal between fat loss, maintenance, and weight gain while in the Advice analysis view.

#### Scenario: Advice goal changes

- **WHEN** the user changes the advice goal in normal mode
- **THEN** the TUI updates the selected goal
- **THEN** the Advice analysis reloads or marks itself ready to reload for that goal

#### Scenario: Advice goal control is used outside Advice view

- **WHEN** the user presses the advice goal control outside the Advice analysis view
- **THEN** the TUI does not change record data

### Requirement: TUI refreshes active analysis independently

The TUI SHALL refresh the active analysis view independently from the records list where appropriate.

#### Scenario: Summary view refreshes

- **WHEN** the user refreshes while Summary is active
- **THEN** the TUI refreshes recent records

#### Scenario: Compare view refreshes

- **WHEN** the user refreshes while Compare is active
- **THEN** the TUI refreshes comparison data

#### Scenario: Advice view refreshes

- **WHEN** the user refreshes while Advice is active
- **THEN** the TUI refreshes advice data for the selected goal

#### Scenario: Record data changes

- **WHEN** the user adds, edits, or deletes a weight record
- **THEN** the TUI refreshes recent records
- **THEN** previously loaded Compare and Advice analyses are invalidated or reloaded before being shown as current

### Requirement: TUI uses semantic colors

The TUI SHALL use semantic colors to improve readability while preserving textual labels for the same meanings.

#### Scenario: Panels are rendered

- **WHEN** the TUI renders panel borders and titles
- **THEN** borders are visually subdued
- **THEN** panel titles are visually distinct from ordinary content

#### Scenario: Current focus is rendered

- **WHEN** the TUI renders the selected record row or active analysis tab
- **THEN** the current item is visually distinct from inactive items

#### Scenario: Status messages are rendered

- **WHEN** the TUI renders loading, success/message, or error states
- **THEN** each state uses a distinct semantic style
- **THEN** the displayed text still identifies the state

#### Scenario: Compare values are rendered

- **WHEN** the TUI renders Compare analysis deltas and value sources
- **THEN** favorable, unfavorable, neutral, filled, direct, and missing values use distinct semantic styles where applicable
- **THEN** source labels such as `direct`, `filled`, and `missing` remain visible

#### Scenario: Advice values are rendered

- **WHEN** the TUI renders Advice analysis data status, trend class, recommendation intensity, or caution
- **THEN** each meaningful state uses a semantic style where applicable
- **THEN** the displayed text still identifies the state and recommendation

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

