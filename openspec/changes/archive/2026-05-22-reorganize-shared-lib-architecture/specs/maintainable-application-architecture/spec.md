## ADDED Requirements

### Requirement: Shared behavior is exposed through a library crate boundary
The system SHALL expose reusable application, domain, and storage behavior through the crate library rather than requiring callers to depend on the binary entry point.

#### Scenario: Binary starts the application
- **WHEN** the compiled `br` binary starts
- **THEN** its entry point delegates to a library-owned CLI startup function

#### Scenario: Shared modules are reused
- **WHEN** CLI commands, TUI flows, or tests need use cases, domain calculations, models, validation, or storage interfaces
- **THEN** they access those modules through the library crate boundary

#### Scenario: Binary remains thin
- **WHEN** contributors inspect `src/main.rs`
- **THEN** they find startup wiring only, not domain calculations, command behavior, storage transport, or TUI state logic

## MODIFIED Requirements

### Requirement: Domain logic is independent from presentation and transport
The system SHALL keep validation, weight comparison, trend analysis, advice rules, and shared application concepts free from CLI parsing, terminal styling, HTTP clients, config file access, and presentation-specific types.

#### Scenario: Domain calculation runs in a unit test
- **WHEN** a unit test exercises comparison, trend, advice, or validation behavior
- **THEN** the behavior runs with plain Rust values and does not require a CLI parser, terminal output capture, config file, or network client

#### Scenario: Output formatting changes
- **WHEN** human-readable terminal formatting is changed
- **THEN** the core domain calculations and storage interface remain unchanged

#### Scenario: Presentation-specific argument types change
- **WHEN** CLI parsing types, Clap derives, or TUI input state types change
- **THEN** shared use cases and domain calculations remain unchanged except for explicit conversions at the presentation boundary

### Requirement: Module boundaries remain readable and extensible
The system SHALL use module names and type names that describe their responsibility in user or application terms, with presentation, application, domain, and storage responsibilities discoverable through the library module structure.

#### Scenario: Contributor locates command behavior
- **WHEN** a contributor needs to modify a command flow
- **THEN** the relevant orchestration, domain behavior, storage implementation, and output formatting are discoverable in separate focused modules

#### Scenario: Contributor adds future behavior
- **WHEN** a contributor adds a future feature that builds on weight records
- **THEN** the contributor can reuse existing domain types, storage interfaces, and output helpers without duplicating Supabase request construction or formatting logic

#### Scenario: Contributor locates presentation behavior
- **WHEN** a contributor needs to modify CLI text command behavior or Ratatui behavior
- **THEN** the contributor can find those presentation concerns under presentation-layer modules without searching through application, domain, or storage modules

### Requirement: Interactive terminal UI remains a presentation layer
The system SHALL keep the Ratatui terminal interface separate from domain calculations, storage transport details, existing text-command output formatting, and CLI-specific parsing types.

#### Scenario: TUI performs record operations
- **WHEN** the TUI adds, updates, deletes, refreshes, or analyzes weight records
- **THEN** it delegates behavior through existing use cases, domain functions, and storage interfaces instead of constructing Supabase requests directly

#### Scenario: TUI behavior is tested
- **WHEN** TUI state transitions, event mapping, or rendering states are tested
- **THEN** tests can exercise the behavior with fake data or test backends without making real network calls

#### Scenario: Existing text output changes
- **WHEN** existing command-line text output formatting changes
- **THEN** TUI rendering remains isolated from those formatting helpers except for shared domain values

#### Scenario: CLI argument parsing changes
- **WHEN** CLI-specific argument parsing types change
- **THEN** TUI state and rendering remain isolated from those types except through shared application or domain values
