## Context

`body-recorder` is currently a binary crate whose entry point declares all modules from `src/main.rs`. The code already has useful internal boundaries: command parsing and dispatch, Ratatui state/rendering, use cases, repository traits, Supabase transport, validation, statistics, and output formatting are mostly separate. The next architectural pressure is that CLI and TUI should both consume the same library-level application behavior without shared logic being owned by the binary entry point or depending on presentation-specific types.

This change is structural. It should preserve existing commands, TUI behavior, Supabase access, config behavior, output text, and tests while making module ownership clearer.

## Goals / Non-Goals

**Goals:**

- Introduce `src/lib.rs` as the reusable crate boundary for shared application behavior.
- Keep `src/main.rs` as a thin binary startup wrapper.
- Organize CLI text commands and TUI under presentation-layer modules.
- Organize reusable use cases, domain calculations, validation, models, repository traits, and storage implementations under library modules.
- Remove application-layer dependencies on CLI-specific types.
- Preserve current user-facing behavior and avoid runtime dependency changes unless required by the reorganization.

**Non-Goals:**

- Do not split the project into a Cargo workspace or multiple packages.
- Do not rewrite command behavior, TUI interaction patterns, Supabase request semantics, or domain algorithms.
- Do not introduce a new persistence backend.
- Do not redesign CLI text output or TUI visuals.

## Decisions

### Use one package with both library and binary targets

Add `src/lib.rs` and move module declarations from `main.rs` into the library. Keep the existing `[[bin]]` target for `br`.

Rationale: the project is small enough that a workspace would add coordination overhead without solving a current problem. A library target gives tests, CLI, and TUI a shared dependency boundary immediately.

Alternative considered: split into crates such as `body-recorder-core`, `body-recorder-cli`, and `body-recorder-tui`. That may become useful later, but it is premature while there is one binary and one deployment artifact.

### Treat CLI and TUI as presentation modules

Group text-command parsing, dispatch, and output rendering under a CLI presentation module. Keep Ratatui state, event mapping, terminal guard, and rendering under a TUI presentation module.

Rationale: CLI and TUI both translate user interaction into application use case calls. Neither should own domain calculations, repository contracts, or storage transport details.

Alternative considered: keep the current flat source layout and only add `lib.rs`. That would technically expose a library but would not make the architecture clearer for future contributors.

### Keep use cases as the application boundary

Use cases should continue to orchestrate validation, repository calls, and domain calculations. Presentation layers call use cases; use cases return plain result types that both CLI and TUI can render.

Rationale: this preserves the existing tested shape and avoids duplicating command behavior across CLI and TUI.

Alternative considered: move orchestration into presentation-specific controllers. That would increase duplication and weaken the repository abstraction.

### Move shared enums out of CLI-specific ownership

Application and domain modules must not depend on Clap-specific CLI types. Any shared concept currently represented by a CLI enum should move to a shared module, while CLI parsing types convert into it.

Rationale: this keeps reusable logic independent from command-line parsing and allows TUI or future callers to use the same concept without depending on Clap.

Alternative considered: keep using CLI enums because they are simple. That is acceptable short term, but it contradicts the intended dependency direction and makes the library boundary less meaningful.

## Risks / Trade-offs

- Module moves can create noisy diffs and broken paths -> keep behavior changes out of the refactor and run `cargo fmt`, `cargo check`, and `cargo test`.
- Public visibility may become too broad when moving to `lib.rs` -> expose only the modules and types needed by the binary, tests, CLI, and TUI.
- Circular dependencies may appear during migration -> move shared types to lower-level `app` or `domain` modules before updating imports.
- Large one-shot moves can obscure regressions -> migrate in small compileable steps: create library boundary, move presentation modules, then move application/domain/storage modules.

## Migration Plan

1. Add `src/lib.rs` and move existing module declarations from `main.rs` into the library.
2. Update `main.rs` to call the library CLI entry point.
3. Move CLI parsing, dispatch, and output rendering under a presentation CLI module.
4. Move TUI modules under a presentation TUI module while preserving its repository-based entry point.
5. Move use cases and result types under an application module.
6. Move models, validation, statistics, and schema helpers under domain-oriented modules.
7. Move repository traits, Supabase implementation, and config access under storage or infrastructure modules.
8. Move shared concepts out of CLI-specific modules and update callers.
9. Run formatting and tests to verify behavior is preserved.

Rollback is straightforward because this is a source organization change: revert the module moves if compile or behavior verification fails.

## Open Questions

- Whether configuration belongs under `storage`, `infra`, or a top-level `config` module can be decided during implementation based on the cleanest dependency direction.
- Whether `schema_sql` is domain contract or storage support can be decided during implementation; it should remain reusable and independent from presentation rendering.
