## Context

The application currently exposes command-oriented flows through Clap, delegates behavior to use-case functions, keeps Supabase behind `WeightRepository`, and renders command output through `src/output.rs`. This structure is a good fit for adding a TUI as a second presentation layer instead of replacing existing commands.

The TUI needs to run inside the existing Tokio application because production storage uses Reqwest async calls. Ratatui itself is a rendering library rather than an async application framework, so the implementation needs an explicit event loop and state model.

## Goals / Non-Goals

**Goals:**

- Add a `br tui` entry point that launches an interactive Ratatui interface.
- Reuse existing use cases, domain calculations, validation, and repository boundaries.
- Keep terminal concerns isolated from command dispatch, domain logic, and Supabase transport.
- Support a practical first workflow: view recent records, refresh, add, edit, delete, and see lightweight trend/advice context.
- Ensure terminal raw mode and alternate screen state are restored on normal exits and errors.
- Test UI state, event mapping, and renderable output without real network calls.

**Non-Goals:**

- Replace or change existing CLI command behavior.
- Add a local database, offline sync, or new storage model.
- Build a full dashboard with every comparison/advice detail from the existing text commands.
- Add mouse-first workflows; keyboard operation is sufficient for the initial interface.

## Decisions

### Add `br tui` instead of changing default behavior

The TUI will be exposed as a new `tui` subcommand. This keeps existing commands stable for scripts and preserves the current command contract.

Alternative considered: launch TUI when no subcommand is provided. That would make interactive use convenient, but it changes the CLI shape and risks surprising existing users or tests.

### Treat TUI as a presentation layer

The TUI will own screen state, focused input fields, selected rows, pending operation state, and rendering. Record operations will still go through use-case functions and `WeightRepository`. Domain statistics will continue to live in `src/stats.rs`.

Alternative considered: call `SupabaseClient` directly from TUI handlers. That would be faster to wire initially, but it would duplicate command behavior and weaken the storage abstraction.

### Use Ratatui with Crossterm

The implementation will use Ratatui's Crossterm backend for cross-platform terminal rendering and keyboard events. Crossterm will also manage raw mode and alternate screen entry/exit.

Alternative considered: use another TUI framework such as Cursive. Ratatui fits the Rust terminal ecosystem well and gives enough control without forcing a different application model.

### Keep network work out of render functions

Rendering functions will derive widgets from current app state only. Refresh, add, edit, and delete actions will update state to a pending status, perform async work outside rendering, then update state with success or error results before the next draw.

Alternative considered: perform repository calls inline wherever the key event is handled. That is simpler for a first draft, but it can make loading states, errors, and redraw timing harder to reason about.

### Use testable state transitions and rendering seams

The TUI should expose small pure functions for key-to-action mapping, state updates, and render output where practical. Tests can use fake repositories and Ratatui's test backend for important screen states.

Alternative considered: rely only on manual terminal testing. Manual testing is still needed for terminal restoration, but it is not enough for regression coverage around state and keyboard behavior.

## Risks / Trade-offs

- Terminal raw mode is not restored after an error -> Use a terminal guard or cleanup path that always disables raw mode and leaves the alternate screen.
- Async repository calls make the event loop more complex -> Keep operation states explicit and start with one pending operation at a time.
- Ratatui and direct Crossterm dependencies can drift to incompatible versions -> Pin compatible versions and check the dependency graph during implementation.
- TUI tests can become brittle if they assert exact layout details -> Prefer testing key text, state transitions, and stable layout intent over full-frame snapshots unless needed.
- The first TUI may duplicate some output concepts from `src/output.rs` -> Keep text command rendering separate from TUI widgets while sharing domain data structures.
