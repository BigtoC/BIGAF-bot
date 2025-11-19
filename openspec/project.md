# Project Context

## Purpose
There is a "vault" product live on MANTRA Chain mainnet, deposit [GAF token](https://blockscout.mantrascan.io/token/0x2ffd3e9f72167743AeE5d044806E711574D7A646) and I'll receive [iGAF token](https://blockscout.mantrascan.io/token/0x94fc3dF643e643AC9294dc2B1C88Bb366587B378), which is a receipt token. So this game is bascially use GAF to buy iGAF, and players can sell iGAF to GAF. In this game, players need to earn as much as GAF, a key feature is there's a random number to determinate the GAF to iGAF exchange rate, the number is set in a smart contract. This gaf-bot project will run on GitHub workflow with a cron, it'll read the last_action_rate.txt get last_action_rate, and call the smart contract get current_rate: if the current_rate > last_action_rate && my iGAF balance > 1, withdraw all (iGAF -> GAF); if current_rate < last_action_rate && GAF balance > 1, deposit all (GAF -> iGAF); otherwise, do nothing. After every action, last_action_rate.txt should be updated.

## Tech Stack
- Rust
- Alloy
- tokio
- tracing
- dotenv
- serde
- serde_json
- reqwest

## Project Conventions

### Code Style
Please write clean Rust code. Use taplo.toml file for code formating.

### Architecture Patterns
- This project should be executed and run once every time, no server or daemon needed.
- Folder structure:
  - In src/contracts folder, put all alloy contract instance files (SimplifiedTeller.rs, AccountantWithRateProviders.rs & erc20.rs)
  - In src/strategies/vault.rs, contains the actual execution logic
  - main.rs is the entry point
- Config via env: private key is set in .env file for local run. In production, private key will be set in GitHub secret and load into environment.
- Constant config: all constant config are set in src/constant.rs

### Testing Strategy
Use `cargo run` in local to test in mainnet. 

### Git Workflow
- Default branch is main; create short-lived feature branches named feat/<topic> or fix/<topic> and rebase before merging.
- Use conventional commits (feat:, fix:, chore:, docs:) so automation and release notes stay consistent.
- Keep OpenSpec proposals/changes in sync with code by landing spec updates in the same PR whenever behavior changes.

## Domain Context
- In last_action_rate.txt, the number unit is wei, which is 18 decimal points, remember to parse it if necessary
- GAF and iGAF are ERC20 tokens, so before deposit or withdraw, approve is needed
- AccountantWithRateProviders is read-only in this use case
- In SimplifiedTeller contract:
  - deposit function: depositAsset is GAF token address, amount is GAF amount in wei, minimumMint need to calculate according to current_rate
  - withdraw function: withdrawAsset is iGAF token address, amount is iGAF amount in wei, minimumAssets need to calculate according to current_rate
- In `accountant_with_rate_providers.rs` and `simplified_teller.rs`, I've already write the required abi, you can directly use them.
- In `erc20.rs`, it's empty, please add all the functions you need.
- Log every step for future debugging, including the amount calculation.

## Important Constraints
[List any technical, business, or regulatory constraints]

## External Dependencies
- `https://evm.dukong.mantrachain.io` - rpc url
