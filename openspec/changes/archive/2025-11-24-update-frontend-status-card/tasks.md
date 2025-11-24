## 1. Implementation
- [x] 1.1 Review the current `Dashboard` status card and confirm available fields from `BotRecord`.
- [x] 1.2 Derive last action rate, action type, current rate, and latest timestamp from the parsed Parquet data.
- [x] 1.3 Update the Current Status card UI to display the derived values plus the new "Last Action Type" row with the required styling.
- [x] 1.4 Implement Bot Status logic that marks the bot **Active** (green) when the latest timestamp is within two hours and **Paused** (yellow) otherwise.

## 2. Validation
- [ ] 2.1 Manually verify the dashboard renders without console errors and that the status card updates as `record.parquet` changes.
- [ ] 2.2 Run `yarn lint` or the equivalent project checks to ensure there are no regressions.
