# Change: Update Frontend Status Card With Live Data

## Why
The dashboard currently shows hard-coded values in the "Current Status" card, which do not reflect the actual bot data recorded in `record.parquet`. This makes it hard to understand the bot's latest action, current rate, or whether the bot is active.

## What Changes
- Source Last Action Rate, Last Action Type, and Current Rate from the latest rows in `record.parquet` instead of static placeholders.
- Compute the Bot Status field based on the most recent timestamp, marking it **Active** (green) if the data is newer than two hours and **Paused** (yellow) otherwise.
- Surface the derived values in the UI so operators can immediately see the bot's current state without digging into logs or the chart.

## Impact
- Affected specs: `frontend`
- Affected code: `frontend/src/components/Dashboard.tsx`, `frontend/src/lib/parquet.ts`
