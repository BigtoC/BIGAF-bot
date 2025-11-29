# Change: Simulate Transaction Before Write

## Why
Currently, the bot attempts to execute transactions directly based on strategy conditions. If a transaction fails on-chain (e.g., due to slippage, insufficient funds for gas, or contract state changes), it consumes gas and results in a failed transaction record. Simulating the transaction locally before broadcasting allows the bot to catch these errors early and avoid unnecessary gas costs.

## What Changes
- The bot will simulate any state-changing transaction (deposit or withdraw) before sending it to the network.
- If the simulation fails, the bot will abort the transaction and record a "hold" action instead of a failed transaction.
- The strategy logic will be updated to include this simulation step as a gatekeeper for write operations.

## Impact
- Affected specs: `bot`
- Affected code: `src/strategies/vault.rs`
