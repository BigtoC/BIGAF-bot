# Add History Table to Frontend

## Summary
Add a data table to the frontend dashboard to display the bot's execution history from the parquet file. The table will support sorting, filtering, and pagination, and will display raw data without rounding.

## Motivation
Users need a detailed view of the bot's history beyond the chart visualization. A table allows for precise inspection of exchange rates, amounts, and transaction hashes.

## Proposed Changes
1.  **Dependencies**: Install `@tanstack/react-table` and `class-variance-authority` (already there), `clsx` (already there), `tailwind-merge` (already there).
2.  **Data Parsing**: Update `frontend/src/lib/parquet.ts` to extract `transaction_hash`, `amount_diff`, and raw string values for rates/amounts to prevent rounding errors.
3.  **UI Components**:
    - Add Shadcn UI `Table` components in `frontend/src/components/ui/table.tsx`.
    - Create `frontend/src/components/HistoryTable.tsx` implementing the table logic (sorting, pagination, formatting).
4.  **Integration**: Add the `HistoryTable` component to `frontend/src/components/Dashboard.tsx` below the chart.

## Design Details
-   **Table Library**: `@tanstack/react-table` for state management (sorting, pagination).
-   **Styling**: Shadcn UI table styles.
-   **Columns**:
    -   Timestamp (Sortable, Default DESC)
    -   Action Type
    -   GAF Amount (Raw string)
    -   Exchange Rate (Raw string)
    -   Amount Diff (Raw string)
    -   Transaction Hash (Hyperlink if starts with 0x)
-   **Pagination**: 10 (default), 20, 50, 100 records per page.
-   **Sorting**: Timestamp sortable (ASC/DESC).

## Alternatives Considered
-   **Simple HTML Table**: Lacks advanced features like sorting and pagination which are required.
-   **DataGrid**: Overkill for this dataset size.
