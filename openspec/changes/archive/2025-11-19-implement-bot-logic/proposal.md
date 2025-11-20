# Change: Implement Bot Logic

## Why
The project is currently empty. We need to implement the core logic for the GAF/iGAF arbitrage bot as described in `openspec/project.md`.

## What Changes
- Implement `src/main.rs` to run the bot logic.
- Implement `src/strategies/vault.rs` for the execution logic.
- Implement `src/contracts/erc20.rs` for ERC20 interactions.
- Implement `src/constant.rs` for configuration.
- Add dependencies to `Cargo.toml`.
- Ensure `last_action_rate.txt` is read and updated.

## Impact
- Affected specs: `bot`
- Affected code: `src/`
