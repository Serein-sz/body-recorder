## ADDED Requirements

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
