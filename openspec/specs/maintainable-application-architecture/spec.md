## Purpose

Define architecture quality expectations for keeping the CLI readable, testable, and extensible as new weight-recording features are added.

## Requirements

### Requirement: Command behavior is organized around use cases

The system SHALL keep CLI parsing and command dispatch separate from the business logic that performs each command.

#### Scenario: Existing command is executed

- **WHEN** a user runs an existing command
- **THEN** the command dispatcher selects the command and delegates the behavior to a focused use-case function or type

#### Scenario: New command behavior is added

- **WHEN** a contributor adds a command that uses existing weight records
- **THEN** the contributor can add the command flow without placing domain calculations, HTTP request construction, and terminal formatting in the dispatcher

### Requirement: Storage access is isolated behind a narrow interface

The system SHALL access persisted weight records through an application-facing storage interface instead of requiring command behavior to construct or call Supabase transport details directly.

#### Scenario: Command logic is tested

- **WHEN** command behavior is tested
- **THEN** the test can provide an in-memory or fake storage implementation without making real network calls

#### Scenario: Supabase storage is used in production

- **WHEN** the CLI runs against the configured Supabase project
- **THEN** the Supabase implementation satisfies the same storage interface used by command behavior

### Requirement: Domain logic is independent from presentation and transport

The system SHALL keep validation, weight comparison, trend analysis, and advice rules free from CLI parsing, terminal styling, HTTP clients, and config file access.

#### Scenario: Domain calculation runs in a unit test

- **WHEN** a unit test exercises comparison, trend, advice, or validation behavior
- **THEN** the behavior runs with plain Rust values and does not require a CLI parser, terminal output capture, config file, or network client

#### Scenario: Output formatting changes

- **WHEN** human-readable terminal formatting is changed
- **THEN** the core domain calculations and storage interface remain unchanged

### Requirement: Refactor preserves existing CLI behavior

The system SHALL preserve the current user-facing behavior for initialization, schema output, weight add/list/update/delete, comparison, and diet advice commands, except for the compare command's specified one-year retrieval boundary and transparent smoothed fill behavior.

#### Scenario: Existing command arguments are used

- **WHEN** a user invokes an existing command with currently supported arguments
- **THEN** the command remains accepted with the same meaning

#### Scenario: Existing data model is used

- **WHEN** commands read or write weight records
- **THEN** they continue to use the existing Supabase `weight_records` table shape and local configuration fields

#### Scenario: Compare behavior is enhanced

- **WHEN** a user invokes the existing `compare` command
- **THEN** the command continues to use the same arguments while applying the one-year retrieval boundary and smoothed fill behavior defined for historical weight comparison

### Requirement: Module boundaries remain readable and extensible

The system SHALL use module names and type names that describe their responsibility in user or application terms.

#### Scenario: Contributor locates command behavior

- **WHEN** a contributor needs to modify a command flow
- **THEN** the relevant orchestration, domain behavior, storage implementation, and output formatting are discoverable in separate focused modules

#### Scenario: Contributor adds future behavior

- **WHEN** a contributor adds a future feature that builds on weight records
- **THEN** the contributor can reuse existing domain types, storage interfaces, and output helpers without duplicating Supabase request construction or formatting logic
