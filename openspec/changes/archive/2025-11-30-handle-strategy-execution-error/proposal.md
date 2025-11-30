# Handle Strategy Execution Error

## Summary
Wrap the strategy execution logic to catch any runtime errors and fallback to a "hold" action instead of crashing the bot. This ensures that transient failures (e.g., RPC issues, simulation failures) are recorded as "hold" actions, keeping the execution history intact and preventing workflow failures.

## Motivation
Currently, if `execute_strategy` encounters an error (e.g., network timeout, simulation revert), the bot crashes and the GitHub workflow fails. This behavior is undesirable for a scheduled bot that should be resilient to transient issues. By catching errors and recording a "hold" action, we ensure the bot completes its run gracefully and logs the failure reason.

## Proposed Changes
1.  Refactor `execute_strategy` in `src/strategies/vault.rs` to wrap the core logic.
2.  Catch any `Err` returned by the core logic.
3.  If an error occurs, log the error and return a `Record` with `action_type: "hold"`.
4.  If the current exchange rate is not available (e.g., error occurred before fetching it), use a default value (e.g., "0.000000000000000000") for `current_exchange_rate`.
5.  Store the error message in `transaction_hash` (or a new field if we were changing schema, but reusing `transaction_hash` for hold reason seems to be the current pattern).

## Alternatives Considered
-   **Retry Logic:** We could implement retries for specific errors, but a "hold" action is a safer default for now.
-   **Panic:** Letting it crash is the current behavior, which we want to avoid.
