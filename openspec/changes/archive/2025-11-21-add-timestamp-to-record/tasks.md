## 1. Implementation
- [ ] 1.1 Add `chrono` to `Cargo.toml`.
- [ ] 1.2 Update `Record` struct in `src/record.rs` to include `timestamp: Option<String>`.
- [ ] 1.3 Update `append_record` in `src/record.rs` to set `timestamp` to current UTC time if not provided.
- [ ] 1.4 Update `get_last_record` in `src/record.rs` to read the `timestamp` column.
- [ ] 1.5 Update `src/strategies/vault.rs` to initialize `Record` with `timestamp: None`.
- [ ] 1.6 Update `enforce_decimal_schema` or add a new schema enforcement to ensure `timestamp` column exists in existing parquet files (or handle schema evolution).
