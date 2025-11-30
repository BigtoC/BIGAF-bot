# Frontend Specification

## ADDED Requirements

### Requirement: History Table
The system SHALL display a tabular view of the bot's execution history below the performance chart.

#### Scenario: Display Columns
- **WHEN** the table renders
- **THEN** it displays columns for Timestamp, Action Type, GAF Amount, Exchange Rate, Amount Diff, and Transaction Hash
- **AND** numeric values are displayed as raw strings without rounding.

#### Scenario: Transaction Hash Link
- **WHEN** a transaction hash starts with "0x"
- **THEN** it is rendered as a hyperlink to `https://blockscout.mantrascan.io/tx/{transaction_hash}`.

#### Scenario: Pagination
- **WHEN** the table renders
- **THEN** it shows 10 records per page by default
- **AND** allows the user to select 20, 50, or 100 records per page
- **AND** displays the current page number and total pages
- **AND** provides buttons to navigate to the first, previous, next, and last pages
- **AND** allows the user to jump to a specific page number.

#### Scenario: Sorting
- **WHEN** the table loads
- **THEN** it is sorted by Timestamp in descending order
- **AND** the user can toggle sorting between ascending and descending order.
