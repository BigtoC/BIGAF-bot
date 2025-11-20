# Change: Update Transaction Buffer

## Why
To avoid potential issues with rounding or dust, or to reserve a small amount of tokens, the user requested that every deposit and withdraw amount should be reduced by 0.1.

## What Changes
- Modify the bot logic to subtract 0.1 (10^17 wei) from the balance before depositing or withdrawing.
- Update the `bot` spec to reflect this buffer.

## Impact
- Affected specs: `bot`
- Affected code: `src/strategies/vault.rs`
