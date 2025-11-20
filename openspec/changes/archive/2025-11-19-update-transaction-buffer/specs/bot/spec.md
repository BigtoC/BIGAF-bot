## MODIFIED Requirements

### Requirement: Bot Execution Strategy
The bot SHALL execute a strategy based on the comparison between the current exchange rate and the last action rate, ensuring a 0.1 token buffer is left remaining.

#### Scenario: Deposit GAF
- **WHEN** current_rate < last_action_rate
- **AND** GAF balance > 0.1 GAF
- **THEN** deposit (GAF balance - 0.1 GAF) to receive iGAF
- **AND** update last_action_rate.txt

#### Scenario: Withdraw iGAF
- **WHEN** current_rate > last_action_rate
- **AND** iGAF balance > 0.1 iGAF
- **THEN** withdraw (iGAF balance - 0.1 iGAF) to receive GAF
- **AND** update last_action_rate.txt

#### Scenario: No Action
- **WHEN** conditions for deposit or withdraw are not met
- **THEN** do nothing
- **AND** do not update last_action_rate.txt
