# Proposal: Check GAF Balance Before Withdrawal

## Summary
This proposal introduces a safety check before executing a withdrawal action. The bot will verify if the iGAF contract holds enough GAF tokens to fulfill the withdrawal request. If the balance is insufficient, the bot will log a warning and skip the withdrawal, recording a `Hold` action instead.

## Why
Currently, the bot attempts to withdraw iGAF (exchange for GAF) based on rate conditions. However, if the iGAF contract does not have enough GAF liquidity, the transaction will fail on-chain, consuming gas unnecessarily. By checking the balance beforehand, we can avoid failed transactions and provide better visibility into why a withdrawal was skipped.

## What Changes
- **Bot Logic**:
  - Before executing a withdrawal, query the GAF token balance of the iGAF contract address.
  - Compare the calculated withdrawal amount (in GAF terms) against the iGAF contract's GAF balance.
  - If `withdrawal_amount_in_gaf > igaf_contract_gaf_balance`, log a warning and return a `Hold` record.
