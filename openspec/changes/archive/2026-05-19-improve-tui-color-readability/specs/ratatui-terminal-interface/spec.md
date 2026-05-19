## ADDED Requirements

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
