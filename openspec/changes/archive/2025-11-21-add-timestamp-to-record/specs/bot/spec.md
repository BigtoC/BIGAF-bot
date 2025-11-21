## MODIFIED Requirements
### Requirement: Execution History Management
The bot SHALL persist the execution history in a structured Parquet file (`record.parquet`) to track rates and actions over time.

#### Scenario: Read Last Rate from History
- **WHEN** the bot starts
- **AND** `record.parquet` exists
- **THEN** read the last row's `current_exchange_rate` to use as `last_action_rate`
- **AND** if the file does not exist, default `last_action_rate` to 0 (or appropriate initial value)

#### Scenario: Append Execution Record
- **WHEN** the bot finishes an execution cycle (Deposit, Withdraw, or Hold)
- **THEN** append a new row to `record.parquet`
- **AND** the row MUST contain: `timestamp` (String, ISO 8601 UTC), `action_type` (String), `gaf_amount` (String decimal with exactly 18 fractional digits, nullable), `current_exchange_rate` (String decimal with exactly 18 fractional digits), `amount_diff` (String decimal with exactly 18 fractional digits, nullable), `transaction_hash` (String, nullable)
- **AND** `gaf_amount` SHALL be `null` when the action type is `hold`, while `transaction_hash` SHALL be `null` for `hold` actions or failures
- **AND** numeric values SHALL be stored as 18-decimal strings without rounding to preserve full precision from on-chain wei amounts
- **AND** `timestamp` SHALL be the current UTC time when the record is saved

#### Scenario: State Persistence via Git
- **WHEN** `record.parquet` is updated
- **THEN** the workflow commits and pushes the change back to the repository (handled by workflow, but file must be updated on disk)
