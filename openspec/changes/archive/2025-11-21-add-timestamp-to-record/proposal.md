# Change: Add Timestamp to Execution Record

## Why
To better track the history of bot actions and analyze performance over time, each execution record needs a precise timestamp. This allows for temporal analysis of exchange rates and bot behavior.

## What Changes
- Add `chrono` dependency for time handling.
- Update `Record` struct to include a `timestamp` field.
- Update `append_record` in `src/record.rs` to populate the `timestamp` field with the current UTC time when saving.
- Update `src/strategies/vault.rs` to accommodate the `Record` struct change (initializing with `None` or letting `record.rs` handle it).
- Update `record.parquet` schema to include the new column.

## Impact
- Affected specs: `bot`
- Affected code: `src/record.rs`, `src/strategies/vault.rs`, `Cargo.toml`
- Data migration: Existing parquet file will need the new column (handled by schema enforcement or new file creation).
