## MODIFIED Requirements
### Requirement: Frontend Dashboard
The system SHALL provide a web-based dashboard to visualize bot data and interact with the blockchain.

#### Scenario: View Dashboard
- **WHEN** a user navigates to the deployed website
- **THEN** the dashboard loads successfully
- **AND** displays a chart using Apache ECharts
- **AND** shows a wallet connection option
- **AND** the Current Status card shows Last Action Rate, Last Action Type, and Current Rate derived from the latest `record.parquet` data
- **AND** the Bot Status field displays **Active** in green text when the newest record timestamp is within two hours of the current time
- **AND** the Bot Status field displays **Paused** in yellow text when the newest record timestamp is older than two hours
