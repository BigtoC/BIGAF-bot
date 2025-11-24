# Proposal: Add Frontend Chart

## Summary
Add a stacked line chart to the frontend dashboard to visualize the bot's historical data from `record.parquet`. The chart will display exchange rates and transaction amounts (deposits/withdrawals) with time range filtering capabilities.

## Motivation
Users need to visualize the bot's performance and historical actions to understand its behavior and profitability. A chart providing exchange rate trends and marking specific deposit/withdraw actions will offer this insight.

## Proposed Changes
1.  **Data Fetching**: Fetch `record.parquet` from the GitHub repository raw URL.
2.  **Data Parsing**: Use a Parquet parsing library (e.g., `apache-arrow` or `hyparquet`) to decode the data in the browser.
3.  **Visualization**: Implement a Stacked Line Chart using Apache ECharts.
    *   **Series 1**: `current_exchange_rate` (Line).
    *   **Series 2**: `gaf_amount` (Line/Scatter), plotting points only for 'deposit' (pink) and 'withdraw' (black) actions.
4.  **Interactivity**: Add a time range selector (24H, 7D, 30D, All) to filter the displayed data.
5. Remove the existing placeholder chart.

## Alternatives Considered
*   **JSON vs Parquet**: The backend already produces Parquet, so the frontend must parse it. Converting to JSON on the backend/workflow side was considered but parsing Parquet in the frontend is more direct given the current architecture.
*   **Chart.js vs ECharts**: ECharts was requested specifically and offers better support for complex mixed charts and zooming.
