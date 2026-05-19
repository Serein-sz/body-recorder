## Why

The `compare` command currently reaches slightly beyond one year so it can average around the one-year comparison point, and sparse records can leave historical comparison rows empty. Users need the command to stay within a clear one-year data boundary while still producing useful, conservative comparisons when a target window has no direct records.

## What Changes

- Limit `compare` data retrieval to records from the reference date back through one year earlier, inclusive.
- Add smoothed fill behavior for comparison points that have no direct records in their target window when nearby records inside the one-year range can support an estimate.
- Preserve explicit no-data handling when a value cannot be filled from available records.
- Show whether each comparison value came from direct records or a smoothed fill so users can judge confidence.
- No breaking changes to existing command arguments or persisted data.

## Capabilities

### New Capabilities
- `historical-weight-comparison`: Covers CLI comparison behavior for one-year historical weight records, smoothed missing comparison values, and transparent output status.

### Modified Capabilities
- `maintainable-application-architecture`: Compare behavior remains organized in use cases, domain calculations, repository access, and output formatting without coupling storage transport to comparison logic.

## Impact

- Affected code: `src/use_cases.rs`, `src/stats.rs`, `src/output.rs`, and related tests.
- Storage/API: reuse the existing `WeightRepository::list_weights_between` operation and Supabase `weight_records` table shape.
- CLI: the `compare` command keeps its current arguments but may show filled comparison values and a clearer source/status for sparse periods.
