## 1. Library Boundary

- [x] 1.1 Add `src/lib.rs` and move reusable module declarations out of `src/main.rs`.
- [x] 1.2 Update `src/main.rs` so it only initializes Tokio, calls the library-owned CLI startup function, reports errors, and exits.
- [x] 1.3 Verify the `br` binary target still builds without changing its command-line interface.

## 2. Presentation Modules

- [x] 2.1 Move CLI parsing, command dispatch, and text output rendering under a CLI presentation module.
- [x] 2.2 Move Ratatui modules under a TUI presentation module while preserving the existing TUI entry point and repository injection.
- [x] 2.3 Update module visibility so presentation internals remain scoped while library callers can access only the intended startup surfaces.

## 3. Shared Application, Domain, and Storage Modules

- [x] 3.1 Move use cases and use-case result types under an application module.
- [x] 3.2 Move models, validation, statistics, and schema helpers under domain-oriented modules.
- [x] 3.3 Move repository traits, Supabase storage implementation, and config access under storage or infrastructure modules based on dependency direction.
- [x] 3.4 Move shared concepts currently owned by CLI parsing, such as advice goal selection, into application or domain modules and convert CLI arguments at the presentation boundary.

## 4. Verification

- [x] 4.1 Update imports and tests after module moves without changing tested behavior.
- [x] 4.2 Run `cargo fmt`.
- [x] 4.3 Run `cargo check`.
- [x] 4.4 Run `cargo test`.
- [x] 4.5 Confirm OpenSpec validation/status reports the change is ready for implementation.
