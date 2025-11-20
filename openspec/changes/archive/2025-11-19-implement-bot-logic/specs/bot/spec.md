## ADDED Requirements

### Requirement: Bot Execution Strategy
The bot SHALL execute a strategy based on the comparison between the current exchange rate and the last action rate.

#### Scenario: Deposit GAF
- **WHEN** current_rate < last_action_rate
- **AND** GAF balance > 1 wei
- **THEN** deposit all GAF to receive iGAF
- **AND** update last_action_rate.txt

#### Scenario: Withdraw iGAF
- **WHEN** current_rate > last_action_rate
- **AND** iGAF balance > 1 wei
- **THEN** withdraw all iGAF to receive GAF
- **AND** update last_action_rate.txt

#### Scenario: No Action
- **WHEN** conditions for deposit or withdraw are not met
- **THEN** do nothing
- **AND** do not update last_action_rate.txt

### Requirement: Rate Management
The bot SHALL persist the rate at which the last action occurred.

#### Scenario: Read Last Rate
- **WHEN** the bot starts
- **THEN** read the rate from `last_action_rate.txt`

#### Scenario: Update Last Rate
- **WHEN** a deposit or withdraw action is successfully completed
- **THEN** write the current rate to `last_action_rate.txt`
