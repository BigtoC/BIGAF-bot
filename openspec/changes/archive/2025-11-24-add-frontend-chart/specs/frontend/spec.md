## ADDED Requirements

### Capability: Bot Data Visualization

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
- **AND** data points are only plotted if `action_type` is 'deposit' or 'withdraw'
- **AND** 'deposit' points are colored Pink
- **AND** 'withdraw' points are colored Black.

### Requirement: Time Range Filtering
The system SHALL allow the user to filter the displayed data by time range.

#### Scenario: Select Time Range
- **WHEN** the user selects a time range (24H, 7 Days, 30 Days, All)
- **THEN** the chart updates to show only data points within that range relative to the latest data point or current time.
