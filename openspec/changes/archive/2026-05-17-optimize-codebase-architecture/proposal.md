## Why

The current CLI is small but already mixes command orchestration, presentation, storage access, validation, and domain analysis in ways that will become harder to read as features are added. Refactoring the architecture now creates clearer ownership boundaries, makes behavior easier to test without network calls, and leaves room for future commands without growing a single command dispatcher into a catch-all module.

## What Changes

- Reorganize the application into explicit layers for CLI parsing, command use cases, domain logic, output formatting, configuration, and Supabase persistence.
- Preserve existing user-facing CLI behavior for initialization, schema output, weight CRUD, comparison, and diet advice.
- Introduce stable interfaces around storage access so command logic can be tested with in-memory or mocked implementations.
- Move presentation-specific formatting out of command orchestration so business rules remain readable and reusable.
- Keep domain models and validation close to the behavior they protect, with names that reflect user concepts rather than transport details.
- Add focused tests around refactored boundaries to guard against regressions while avoiding real network calls.
- No breaking changes to existing commands, arguments, output intent, or Supabase schema.

## Capabilities

### New Capabilities

- `maintainable-application-architecture`: Define architectural quality requirements for readable module boundaries, testable command flows, and extensible feature organization.

### Modified Capabilities

None.

## Impact

- Code organization: affects `src/commands.rs`, `src/stats.rs`, `src/supabase.rs`, `src/models.rs`, `src/config.rs`, `src/validation.rs`, and potentially new modules under `src/`.
- Tests: adds or adjusts unit tests around use cases, formatting, validation, and storage abstractions.
- APIs: no public CLI contract changes are expected.
- Dependencies: no new external dependencies are expected unless a small test utility is clearly justified.
- Storage: reuses the existing Supabase table and SQL schema; no database migration is expected.
