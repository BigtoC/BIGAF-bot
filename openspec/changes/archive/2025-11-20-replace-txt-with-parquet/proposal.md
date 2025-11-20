# Change: Replace Text File with Parquet Record

## Why
The current system only stores the last action rate in a text file, which loses historical context and makes analysis difficult. Moving to a structured Parquet file allows for recording detailed execution history, including action types, amounts, and rate changes over time.

## What Changes
- Replace `last_action_rate.txt` with `record.parquet`.
- Introduce `polars` for data manipulation and Parquet I/O.
- Record detailed execution data: action type, GAF amount, current exchange rate, transaction_hash and amount difference.
- Enhance logging for better observability.

## Impact
- **Affected specs**: `bot`
- **Affected code**: `src/strategies/vault.rs`, `src/main.rs`, new `src/record.rs`.
- **Data**: `last_action_rate.txt` will be deprecated and removed.
