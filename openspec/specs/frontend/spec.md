# frontend Specification

## Purpose
TBD - created by archiving change add-frontend-dashboard. Update Purpose after archive.
## Requirements
### Requirement: Frontend Dashboard
The system SHALL provide a web-based dashboard to visualize bot data and interact with the blockchain.

#### Scenario: View Dashboard
- **WHEN** a user navigates to the deployed website
- **THEN** the dashboard loads successfully
- **AND** displays a chart using Apache ECharts
- **AND** shows a wallet connection option

### Requirement: Automated Deployment
The system SHALL automatically deploy the frontend to GitHub Pages upon changes to the `frontend/` directory on the `main` branch.

#### Scenario: Push to Main
- **WHEN** code is pushed to the `main` branch
- **AND** the `frontend/` directory has changes
- **THEN** the GitHub Actions workflow triggers
- **AND** builds the project
- **AND** deploys the artifacts to GitHub Pages

### Requirement: Theme Support
The system SHALL allow users to switch between light and dark visual themes.

#### Scenario: Switch to Dark Mode
- **WHEN** the user selects "Dark" from the theme toggle
- **THEN** the application interface changes to dark colors
- **AND** the preference is saved

#### Scenario: Switch to Light Mode
- **WHEN** the user selects "Light" from the theme toggle
- **THEN** the application interface changes to light colors
- **AND** the preference is saved

#### Scenario: System Preference
- **WHEN** the user selects "System" from the theme toggle
- **THEN** the application interface matches the operating system's color scheme

### Requirement: Fetch and Parse Parquet Data
The system SHALL fetch the `record.parquet` file from the configured URL and parse it into a usable format in the frontend.

#### Scenario: Load Data on Init
- **WHEN** the Dashboard component mounts
- **THEN** the application fetches `record.parquet` from `https://github.com/BigtoC/BIGAF-bot/raw/refs/heads/main/record.parquet`
- **AND** parses the binary content into a list of records containing `timestamp`, `current_exchange_rate`, `gaf_amount`, and `action_type`.

### Requirement: Render Stacked Line Chart
The system SHALL render a chart using ECharts with two specific series based on the parsed data.

#### Scenario: Display Exchange Rate
- **WHEN** the chart renders
- **THEN** the first series displays `current_exchange_rate` as a line
- **AND** the original decimal precision is preserved.

#### Scenario: Display Action Amounts
- **WHEN** the chart renders
- **THEN** the second series displays `gaf_amount`
- **AND** the series maintains the last known value between actions so the line never drops to zero between transactions
- **AND** 'deposit' points are colored Pink
- **AND** 'withdraw' points are colored Black.

### Requirement: Time Range Filtering
The system SHALL allow the user to filter the displayed data by time range.

#### Scenario: Select Time Range
- **WHEN** the user selects a time range (24H, 7 Days, 30 Days, All)
- **THEN** the chart updates to show only data points within that range relative to the latest data point or current time.

