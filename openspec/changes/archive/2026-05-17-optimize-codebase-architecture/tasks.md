## 1. Characterization And Boundaries

- [x] 1.1 Run the existing test suite to capture the current baseline before refactoring
- [x] 1.2 Identify current command flows and document the data each flow needs from configuration, storage, domain logic, and output formatting
- [x] 1.3 Add focused characterization tests for user-visible command results where current coverage would not catch a refactor regression

## 2. Storage Interface

- [x] 2.1 Define a narrow application-facing weight record storage trait with operations for upsert, list, list between dates, update, and delete
- [x] 2.2 Move Supabase-specific request construction behind an implementation of the storage trait
- [x] 2.3 Add an in-memory or fake storage implementation for command/use-case tests
- [x] 2.4 Verify command behavior tests do not perform real network calls

## 3. Use Case Layer

- [x] 3.1 Extract add, list, update, delete, compare, and advice behavior into focused use-case functions or types
- [x] 3.2 Keep CLI parsing and top-level command dispatch responsible only for parsing, dependency construction, delegation, and error propagation
- [x] 3.3 Ensure use cases accept explicit inputs and dependencies rather than reading config or constructing Supabase clients internally
- [x] 3.4 Preserve existing validation behavior for dates and weight values

## 4. Presentation Separation

- [x] 4.1 Move comparison output formatting into an output-focused module or clearly named formatting functions
- [x] 4.2 Move diet advice output formatting into an output-focused module or clearly named formatting functions
- [x] 4.3 Keep terminal coloring and layout decisions out of domain calculation modules
- [x] 4.4 Add or update tests for formatting that is important to user-visible behavior

## 5. Domain And Module Cleanup

- [x] 5.1 Keep comparison and trend analysis independent from CLI, config, HTTP, and terminal styling
- [x] 5.2 Rename or relocate transport payload types so domain records and Supabase request bodies have clear ownership
- [x] 5.3 Remove duplicated orchestration or formatting code introduced during extraction
- [x] 5.4 Review module names and public visibility so only intentional APIs are exposed across modules

## 6. Verification

- [x] 6.1 Run `cargo fmt`
- [x] 6.2 Run `cargo test`
- [x] 6.3 Run `cargo check`
- [x] 6.4 Run `cargo clippy --all-targets --all-features` if the refactor changes shared module boundaries
- [x] 6.5 Confirm OpenSpec requirements for `maintainable-application-architecture` are satisfied before archiving
