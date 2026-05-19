## MODIFIED Requirements

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
