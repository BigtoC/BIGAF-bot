## ADDED Requirements

### Requirement: Release Management
The system SHALL automatically build and release the binary artifact upon version tagging.

#### Scenario: Build and Release
- **WHEN** a new tag is pushed (e.g., `v1.0.0`)
- **THEN** build the Rust binary in release mode
- **AND** create a GitHub Release with the tag name
- **AND** upload the binary to the release assets

## MODIFIED Requirements

### Requirement: Automated Execution
The bot SHALL execute automatically on a defined schedule using the pre-built binary from the latest GitHub Release.

#### Scenario: Scheduled Run
- **WHEN** the cron schedule triggers (e.g., hourly)
- **THEN** download the binary from the latest GitHub Release
- **AND** execute the binary
- **AND** pass `PRIVATE_KEY` and other env vars

#### Scenario: State Persistence
- **WHEN** `last_action_rate.txt` is updated during execution
- **THEN** the workflow commits and pushes the change back to the repository
