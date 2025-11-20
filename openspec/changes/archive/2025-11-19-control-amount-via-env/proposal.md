# Change: Control Amount via Env

## Why
The user wants more granular control over the amount of tokens deposited or withdrawn during each action, rather than always transacting the full balance (minus buffer).

## What Changes
- Introduce a new environment variable `ACTION_AMOUNT_CONTROL`.
- Update `src/strategies/vault.rs` to interpret this variable:
    - If `1`: Transact 100% of the balance (minus buffer).
    - If `< 1`: Transact that percentage of the balance (e.g., 0.5 = 50%).
    - If `> 1`: Transact that specific fixed amount of tokens.
- Update `src/constant.rs` if needed (though env vars are usually runtime).
- Update `bot` spec to reflect this new logic.

## Impact
- Affected specs: `bot`
- Affected code: `src/strategies/vault.rs`, `src/main.rs` (to load env)
