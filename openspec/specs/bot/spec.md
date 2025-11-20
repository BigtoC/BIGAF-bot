# bot Specification

## Purpose
TBD - created by archiving change implement-bot-logic. Update Purpose after archive.
## Requirements
### Requirement: Bot Execution Strategy
The bot SHALL execute a strategy based on the comparison between the current exchange rate and the last action rate, determining the transaction amount via `ACTION_AMOUNT_CONTROL` and ensuring a 0.1 token buffer is left remaining when calculating from balance.

#### Scenario: Deposit GAF (Full)
- **WHEN** current_rate < last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` is 1
- **AND** GAF balance > 0.1 GAF
- **THEN** deposit (GAF balance - 0.1 GAF)

#### Scenario: Deposit GAF (Percentage)
- **WHEN** current_rate < last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` < 1 (e.g., 0.5)
- **AND** GAF balance > 0.1 GAF
- **THEN** deposit ((GAF balance - 0.1 GAF) * percentage)

#### Scenario: Deposit GAF (Fixed Amount)
- **WHEN** current_rate < last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` > 1 (e.g., 100)
- **AND** GAF balance >= (Fixed Amount + 0.1 GAF)
- **THEN** deposit Fixed Amount

#### Scenario: Withdraw iGAF (Full)
- **WHEN** current_rate > last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` is 1
- **AND** iGAF balance > 0.1 iGAF
- **THEN** withdraw (iGAF balance - 0.1 iGAF)

#### Scenario: Withdraw iGAF (Percentage)
- **WHEN** current_rate > last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` < 1
- **AND** iGAF balance > 0.1 iGAF
- **THEN** withdraw ((iGAF balance - 0.1 iGAF) * percentage)

#### Scenario: Withdraw iGAF (Fixed Amount)
- **WHEN** current_rate > last_action_rate
- **AND** `ACTION_AMOUNT_CONTROL` > 1
- **AND** iGAF balance >= (Fixed Amount + 0.1 iGAF)
- **THEN** withdraw Fixed Amount

#### Scenario: No Action
- **WHEN** conditions for deposit or withdraw are not met
- **THEN** do nothing

### Requirement: Automated Execution
The bot SHALL execute automatically on a defined schedule using GitHub Actions.

#### Scenario: Scheduled Run
- **WHEN** the cron schedule triggers (e.g., hourly)
- **THEN** the bot executes the strategy
- **AND** uses the `PRIVATE_KEY` from GitHub Secrets

#### Scenario: State Persistence
- **WHEN** `last_action_rate.txt` is updated during execution
- **THEN** the workflow commits and pushes the change back to the repository

### Requirement: Execution History Management
The bot SHALL persist the execution history in a structured Parquet file (`record.parquet`) to track rates and actions over time.

#### Scenario: Read Last Rate from History
- **WHEN** the bot starts
- **AND** `record.parquet` exists
- **THEN** read the last row's `current_exchange_rate` to use as `last_action_rate`
- **AND** if the file does not exist, default `last_action_rate` to 0 (or appropriate initial value)

- **WHEN** the bot finishes an execution cycle (Deposit, Withdraw, or Hold)
- **THEN** append a new row to `record.parquet`
- **AND** the row MUST contain: `action_type` (String), `gaf_amount` (String decimal with exactly 18 fractional digits, nullable), `current_exchange_rate` (String decimal with exactly 18 fractional digits), `amount_diff` (String decimal with exactly 18 fractional digits, nullable), `transaction_hash` (String, nullable)
- **AND** `gaf_amount` SHALL be `null` when the action type is `hold`, while `transaction_hash` SHALL be `null` for `hold` actions or failures
- **AND** numeric values SHALL be stored as 18-decimal strings without rounding to preserve full precision from on-chain wei amounts

#### Scenario: State Persistence via Git
- **WHEN** `record.parquet` is updated
- **THEN** the workflow commits and pushes the change back to the repository (handled by workflow, but file must be updated on disk)

### Requirement: Enhanced Logging
The bot SHALL log detailed information about its decision-making process and state changes to standard output.

#### Scenario: Log Execution Details
- **WHEN** the bot runs
- **THEN** log the `current_rate`, `last_action_rate`, calculated `amount_diff`, the decision (`action_type`), and the action summary returned from the strategy
- **AND** log the resulting transaction hash (if a transaction was sent)

