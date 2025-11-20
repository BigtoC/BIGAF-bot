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

### Requirement: Rate Management
The bot SHALL persist the rate at which the last action occurred.

#### Scenario: Read Last Rate
- **WHEN** the bot starts
- **THEN** read the rate from `last_action_rate.txt`

#### Scenario: Update Last Rate
- **WHEN** a deposit or withdraw action is successfully completed
- **THEN** write the current rate to `last_action_rate.txt`

### Requirement: Automated Execution
The bot SHALL execute automatically on a defined schedule using GitHub Actions.

#### Scenario: Scheduled Run
- **WHEN** the cron schedule triggers (e.g., hourly)
- **THEN** the bot executes the strategy
- **AND** uses the `PRIVATE_KEY` from GitHub Secrets

#### Scenario: State Persistence
- **WHEN** `last_action_rate.txt` is updated during execution
- **THEN** the workflow commits and pushes the change back to the repository

