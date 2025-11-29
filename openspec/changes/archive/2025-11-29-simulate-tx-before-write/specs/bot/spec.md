## ADDED Requirements
### Requirement: Transaction Simulation
The bot SHALL simulate all state-changing transactions (deposit, withdraw) before broadcasting them to the network.

#### Scenario: Simulation Success
- **WHEN** the strategy determines a write action is needed
- **AND** the transaction simulation succeeds
- **THEN** the bot broadcasts the transaction to the network

#### Scenario: Simulation Failure
- **WHEN** the strategy determines a write action is needed
- **AND** the transaction simulation fails (e.g., reverts)
- **THEN** the bot aborts the transaction
- **AND** records a "hold" action
