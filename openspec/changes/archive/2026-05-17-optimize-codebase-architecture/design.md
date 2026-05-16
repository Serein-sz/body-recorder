## Context

`body-recorder` is a Rust CLI that records weight data in Supabase, prints schema SQL, compares historical averages, and produces trend-based diet advice. The codebase already has several modules, but command handling still performs orchestration, storage construction, validation, domain calls, and output formatting in the same file. Supabase access is also concrete at call sites, which makes command-level tests harder without real network calls.

The refactor must improve readability and extension points while preserving the current CLI contract and avoiding database or dependency churn. This is an internal architecture change: users should see the same commands and intent, but contributors should see clearer module responsibilities and lower-cost tests.

## Goals / Non-Goals

**Goals:**

- Separate command orchestration from domain logic, storage access, and presentation formatting.
- Introduce a narrow storage abstraction for weight records so use cases can be tested without Supabase.
- Keep domain calculations deterministic and independent from CLI parsing, HTTP clients, and terminal coloring.
- Make each command flow readable as a short use case: parse input, validate, call dependencies, return printable output.
- Preserve existing behavior and output intent for all current commands.
- Add focused tests at the new boundaries without using real network calls.

**Non-Goals:**

- Change CLI command names, arguments, or Supabase schema.
- Replace Supabase or add a second persistence backend.
- Add a large framework, dependency injection container, or generic architecture layer beyond the needs of this binary.
- Redesign diet advice formulas or comparison semantics.
- Rewrite all modules solely for naming preference when current ownership is already clear.

## Decisions

1. Use a small application/use-case layer for command behavior.

   Command dispatch will remain responsible for selecting the requested command, but each command should delegate to focused use-case functions that accept validated inputs and explicit dependencies. This keeps the dispatcher short and makes future commands easier to add.

   Alternative considered: keep all command functions in one dispatcher file and only split formatting. That is lower effort, but it leaves the main growth pressure in the same module that already coordinates too many concerns.

2. Introduce a `WeightRepository`-style trait around record persistence.

   The Supabase client will implement the storage behavior needed by current commands: upsert, list, list between dates, update, and delete. Use cases can depend on this narrow trait instead of constructing `SupabaseClient` directly.

   Alternative considered: mock `reqwest` or Supabase HTTP calls in command tests. That couples tests to transport details and makes command behavior harder to read.

3. Keep domain modules transport-free and presentation-free.

   Weight comparison, trend analysis, validation, and advice rules should operate on Rust types and return structured results. Terminal coloring, table layout, and user-facing strings should live in output-focused code.

   Alternative considered: keep formatting next to calculation for convenience. That is simple for the current feature count, but it makes future reuse and testing more expensive.

4. Prefer explicit modules over a broad generic architecture.

   New modules should reflect the CLI's current concepts, such as commands/use cases, storage, output, and domain stats. Avoid abstract names like `service` or `manager` unless the surrounding code gives them a precise meaning.

   Alternative considered: adopt a full clean-architecture folder hierarchy. That adds ceremony for a small binary and can obscure rather than clarify ownership.

5. Refactor incrementally with behavior-preserving tests.

   The implementation should move code in small steps, adding tests where boundaries change. Existing unit tests should continue to pass, and command-level behavior should be covered through in-memory storage where practical.

   Alternative considered: perform a broad file move first and test after. That increases regression risk and makes review harder.

## Risks / Trade-offs

- Refactor changes output accidentally -> add focused tests for command results or formatting where behavior is user-visible.
- Trait abstraction becomes too broad -> define only operations current use cases need and expand when a new command requires it.
- More files make navigation harder -> keep module names concrete and avoid one-function modules unless they represent a real boundary.
- Async trait usage can add complexity -> reuse the existing `async-trait` dependency only at the repository boundary and keep domain functions synchronous.
- Internal architecture specs can be subjective -> express requirements as observable codebase properties and testability constraints rather than personal style preferences.
