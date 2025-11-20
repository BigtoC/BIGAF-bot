## ADDED Requirements

### Requirement: Automated Execution
The bot SHALL execute automatically on a defined schedule using GitHub Actions.

#### Scenario: Scheduled Run
- **WHEN** the cron schedule triggers (e.g., hourly)
- **THEN** the bot executes the strategy
- **AND** uses the `PRIVATE_KEY` from GitHub Secrets

#### Scenario: State Persistence
- **WHEN** `last_action_rate.txt` is updated during execution
- **THEN** the workflow commits and pushes the change back to the repository
