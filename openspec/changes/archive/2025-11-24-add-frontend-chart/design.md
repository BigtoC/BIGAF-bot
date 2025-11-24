# Design: Frontend Chart Architecture

## Data Flow
1.  **Source**: `https://github.com/BigtoC/BIGAF-bot/raw/refs/heads/main/record.parquet`
2.  **Fetch**: `fetch()` API in browser.
3.  **Parse**: `apache-arrow` (or equivalent) reads the ArrayBuffer and converts it to a JSON-like array of objects.
4.  **Transform**:
    *   Filter by selected Time Range.
    *   Map to ECharts `dataset` or `series` format.
    *   `current_exchange_rate` -> Line Series.
    *   `gaf_amount` + `action_type` -> Scatter/Line Series with custom itemStyles for colors (Pink for Deposit, Black for Withdraw).

## Chart Configuration
*   **X-Axis**: Timestamp (Time scale).
*   **Y-Axis**:
    *   Left: Exchange Rate (dynamic scale, don't start at 0).
    *   Right: Amount (optional, or share axis if scales are compatible, but likely need dual axis or normalized view if ranges differ vastly). *Correction*: The prompt asks for a "Stacked Line Chart". Usually, this implies sharing the same axis or stacking values. However, "Exchange Rate" and "Amount" have different units. A "Dual Y-Axis" chart is more appropriate for distinct metrics, but "Stacked" was requested. I will implement it as a Dual Y-Axis chart if the values are vastly different, or a single axis if they are comparable. Given the prompt says "Stacked Line Chart", I will try to stack them or overlay them. *Re-reading prompt*: "the first line is current_exchange_rate... the second line is gaf_amount". I will use two series. If the user insists on "Stacked", ECharts has a `stack` property, but stacking rate and amount makes little sense physically. I will assume "Superimposed" (overlay) is what is meant, potentially with dual axes if needed, or just two lines. I'll stick to the user's "Stacked Line Chart" request literally if possible, but overlay is safer for visibility. I'll configure it as two line series.

## Libraries
*   `echarts`: Core charting.
*   `echarts-for-react`: React wrapper.
*   `apache-arrow`: For parsing Parquet files in the browser.
