## ADDED Requirements

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
