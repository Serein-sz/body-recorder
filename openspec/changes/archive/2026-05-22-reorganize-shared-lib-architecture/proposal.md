## Why

The project already separates command dispatch, TUI state, use cases, storage, and domain calculations, but those modules are still rooted under the binary entry point. Moving shared behavior behind a library crate boundary will make the CLI and TUI clearer presentation layers and reduce coupling as more features reuse the same application logic.

## What Changes

- Add a reusable library crate entry point for shared application, domain, and storage modules.
- Keep the binary entry point thin, delegating startup to the CLI presentation layer.
- Reorganize source modules so CLI text commands and Ratatui remain presentation concerns while use cases, models, validation, statistics, repository traits, and storage implementations are reusable from the library.
- Remove presentation-to-application coupling leaks, such as shared use cases depending on CLI-specific argument types.
- Preserve existing CLI commands, TUI behavior, Supabase table shape, config behavior, and user-facing output.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `maintainable-application-architecture`: Require a reusable library boundary and explicit presentation-layer organization for CLI and TUI.

## Impact

- Affected code: `src/main.rs`, module declarations, CLI dispatch/output modules, TUI modules, use cases, domain modules, repository/storage modules, and tests that reference module paths.
- Public behavior: no intended user-facing command, TUI, schema, config, or storage behavior changes.
- Dependencies: no new runtime dependency expected.
