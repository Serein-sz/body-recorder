## ADDED Requirements

### Requirement: Interactive terminal UI remains a presentation layer

The system SHALL keep the Ratatui terminal interface separate from domain calculations, storage transport details, and existing text-command output formatting.

#### Scenario: TUI performs record operations

- **WHEN** the TUI adds, updates, deletes, refreshes, or analyzes weight records
- **THEN** it delegates behavior through existing use cases, domain functions, and storage interfaces instead of constructing Supabase requests directly

#### Scenario: TUI behavior is tested

- **WHEN** TUI state transitions, event mapping, or rendering states are tested
- **THEN** tests can exercise the behavior with fake data or test backends without making real network calls

#### Scenario: Existing text output changes

- **WHEN** existing command-line text output formatting changes
- **THEN** TUI rendering remains isolated from those formatting helpers except for shared domain values
