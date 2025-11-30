# Bot Specification Changes

## MODIFIED Requirements

### Requirement: Bot Execution Strategy
The bot SHALL execute a strategy based on the comparison between the current exchange rate and the last action rate, determining the transaction amount via `ACTION_AMOUNT_CONTROL` and ensuring a 0.1 token buffer is left remaining when calculating from balance.

#### Scenario: Insufficient Liquidity for Withdrawal
- **WHEN** conditions for withdrawal are met (current_rate > last_action_rate)
- **AND** the iGAF contract's GAF balance is less than the estimated GAF amount required for the withdrawal
- **THEN** log a warning indicating insufficient liquidity
- **AND** do not execute the withdrawal transaction
- **AND** record a `Hold` action
