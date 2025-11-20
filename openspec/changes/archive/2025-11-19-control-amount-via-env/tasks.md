## 1. Implementation
- [x] 1.1 Add `ACTION_AMOUNT_CONTROL` to `.env` (example) and `src/constant.rs` (default).
- [x] 1.2 Update `src/strategies/vault.rs` to read `ACTION_AMOUNT_CONTROL`.
- [x] 1.3 Implement logic to calculate `amount_to_transact` based on the control value (1, <1, >1).
- [x] 1.4 Ensure the calculated amount respects the balance and buffer (don't overdraw).
