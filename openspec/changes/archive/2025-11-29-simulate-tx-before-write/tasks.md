## 1. Implementation
- [x] 1.1 Update `src/strategies/vault.rs` to include transaction simulation logic using `call()` or `estimate_gas()` before sending `deposit` or `withdraw` transactions.
- [x] 1.2 Modify the execution flow to catch simulation errors.
- [x] 1.3 If simulation fails, return a `hold_record` with the current rate, effectively skipping the action for this run.

## 2. Validation
- [x] 2.1 Verify that valid transactions still go through.
- [x] 2.2 Verify that transactions that would fail (e.g., by mocking a failure or using a state that causes revert) result in a "hold" record and no on-chain transaction.
