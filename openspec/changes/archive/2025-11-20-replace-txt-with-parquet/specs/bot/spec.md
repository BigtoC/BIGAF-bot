## RENAMED Requirements
- FROM: `### Requirement: Rate Management`
- TO: `### Requirement: Execution History Management`

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
- **AND** the row MUST contain: `action_type` (String), `gaf_amount` (Float64, nullable), `current_exchange_rate` (Float64), `amount_diff` (Float64, nullable), `transaction_hash` (String, nullable)
- **AND** `gaf_amount` SHALL be `null` when the action type is `hold`, while `transaction_hash` SHALL be `null` for `hold` actions or failures

#### Scenario: State Persistence via Git
- **WHEN** `record.parquet` is updated
- **THEN** the workflow commits and pushes the change back to the repository (handled by workflow, but file must be updated on disk)

## ADDED Requirements
### Requirement: Enhanced Logging
The bot SHALL log detailed information about its decision-making process and state changes to standard output.

#### Scenario: Log Execution Details
- **WHEN** the bot runs
- **THEN** log the `current_rate`, `last_action_rate`, calculated `amount_diff`, the decision (`action_type`), and the action summary returned from the strategy
- **AND** log the resulting transaction hash (if a transaction was sent)
