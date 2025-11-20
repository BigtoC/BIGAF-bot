## 1. Implementation
- [x] 1.1 Add `polars` (with `parquet` feature) to `Cargo.toml`.
- [x] 1.2 Create `src/record.rs` to handle reading and appending to `record.parquet`.
- [x] 1.3 Implement `Record` struct and schema definition (action_type, gaf_amount, current_exchange_rate, amount_diff, transaction_hash).
- [x] 1.4 Update `src/strategies/vault.rs` to read the last rate from `record.parquet` (if exists) or default.
- [x] 1.5 Update `src/strategies/vault.rs` to append new execution details to `record.parquet` after action.
- [x] 1.6 Add detailed `info!` logs throughout the execution flow in `src/strategies/vault.rs`.
- [x] 1.7 Remove `last_action_rate.txt` reading/writing logic.
- [x] 1.8 Delete `last_action_rate.txt`.
