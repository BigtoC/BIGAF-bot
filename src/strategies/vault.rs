use crate::constant::{
    ACCOUNTANT_WITH_RATE_PROVIDERS_ADDRESS, DEFAULT_ACTION_AMOUNT_CONTROL,
    DEFAULT_MAX_DEPOSIT_RATE, DEFAULT_MIN_WITHDRAW_RATE, GAF_TOKEN_ADDRESS, IGAF_TOKEN_ADDRESS,
    SIMPLIFIED_TELLER_ADDRESS,
};
use crate::contracts::{
    accountant_with_rate_providers::AccountantWithRateProviders, erc20::ERC20,
    simplified_teller::SimplifiedTeller,
};
use crate::record::{self, Record};
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::{Result, anyhow};
use chrono::Utc;
use core::panic;
use std::env;
use std::str::FromStr;
use tracing::info;
use url::Url;

const WEI_SCALE: u128 = 1_000_000_000_000_000_000;
const WEI_SCALE_F64: f64 = 1_000_000_000_000_000_000.0;

fn available_after_buffer(balance: U256, buffer: U256) -> Option<U256> {
    (balance > buffer).then(|| balance - buffer)
}

fn calculate_action_amount(available_balance: U256, action_amount_control: f64) -> U256 {
    if (action_amount_control - 1.0).abs() < f64::EPSILON {
        return available_balance;
    }

    let precision = U256::from(WEI_SCALE);
    if action_amount_control < 1.0 {
        let factor = U256::from((action_amount_control * WEI_SCALE_F64) as u128);
        available_balance
            .checked_mul(factor)
            .unwrap_or(U256::ZERO)
            .checked_div(precision)
            .unwrap_or(U256::ZERO)
    } else {
        let fixed_amount = U256::from((action_amount_control * WEI_SCALE_F64) as u128);
        if available_balance >= fixed_amount {
            fixed_amount
        } else {
            U256::ZERO
        }
    }
}

fn calculate_minimum_assets(amount: U256, rate: U256) -> U256 {
    let precision = U256::from(WEI_SCALE);
    amount
        .checked_mul(rate)
        .unwrap_or(U256::ZERO)
        .checked_div(precision)
        .unwrap_or(U256::ZERO)
}

fn calculate_minimum_mint(amount: U256, rate: U256) -> U256 {
    let precision = U256::from(WEI_SCALE);
    amount
        .checked_mul(precision)
        .unwrap_or(U256::ZERO)
        .checked_div(rate)
        .unwrap_or(U256::ZERO)
}

fn wei_to_decimal_string(value: U256) -> String {
    let precision = U256::from(WEI_SCALE);
    let integer = value / precision;
    let fraction = value % precision;
    format!("{}.{}", integer, format!("{fraction:018}"))
}

fn decimal_string_to_wei(value: &str) -> Result<U256> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(U256::ZERO);
    }

    if trimmed.starts_with('-') {
        return Err(anyhow!("Negative decimals are not supported: {value}"));
    }

    let (int_part, frac_part) = trimmed
        .split_once('.')
        .map(|(int_part, frac_part)| (int_part, frac_part))
        .unwrap_or((trimmed, ""));

    let integer = if int_part.is_empty() {
        U256::ZERO
    } else {
        int_part
            .parse::<U256>()
            .map_err(|_| anyhow!("Invalid decimal integer part: {value}"))?
    };

    let truncated_frac = &frac_part[..frac_part.len().min(18)];
    let mut fractional = truncated_frac.to_string();
    if !fractional.is_empty() && fractional.chars().any(|c| !c.is_ascii_digit()) {
        return Err(anyhow!("Invalid decimal fractional part: {value}"));
    }
    while fractional.len() < 18 {
        fractional.push('0');
    }

    let fractional_value = if fractional.trim().is_empty() {
        U256::ZERO
    } else {
        fractional
            .parse::<U256>()
            .map_err(|_| anyhow!("Invalid decimal fractional part: {value}"))?
    };

    let precision = U256::from(WEI_SCALE);
    let scaled_integer = integer
        .checked_mul(precision)
        .ok_or_else(|| anyhow!("Decimal {value} exceeds supported range"))?;

    Ok(scaled_integer + fractional_value)
}

fn signed_decimal_difference(current: U256, previous: U256) -> String {
    if current >= previous {
        wei_to_decimal_string(current - previous)
    } else {
        let diff = previous - current;
        format!("-{}", wei_to_decimal_string(diff))
    }
}

fn hold_record(current_rate: String) -> Record {
    Record {
        timestamp: Option::from(Utc::now().to_rfc3339()),
        action_type: "hold".to_string(),
        gaf_amount: None,
        current_exchange_rate: current_rate,
        amount_diff: None,
        transaction_hash: None,
    }
}

pub async fn execute_strategy(rpc_url: &str, private_key: &str) -> Result<Record> {
    // Setup provider
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(Url::parse(rpc_url)?);

    let my_address = signer.address();
    info!("Running bot with address: {}", my_address);

    // Read last action rate from parquet
    let last_record = record::get_last_non_hold_record()?;
    let (last_action_rate, last_rate_decimal) = if let Some(r) = &last_record {
        info!("Found last non hold record: {:?}", r);
        (
            decimal_string_to_wei(&r.current_exchange_rate)?,
            r.current_exchange_rate.clone(),
        )
    } else {
        panic!("No previous record found");
    };

    let last_action_type = &last_record.unwrap().action_type;
    info!(
        "Last action is {last_action_type}, rate in wei: {}",
        last_action_rate
    );
    info!(
        "Last action is {last_action_type}, rate with decimals: {}",
        last_rate_decimal
    );

    // Get current rate
    let accountant_address = Address::from_str(ACCOUNTANT_WITH_RATE_PROVIDERS_ADDRESS)?;
    let accountant = AccountantWithRateProviders::new(accountant_address, provider.clone());
    let current_rate = accountant.getRate().call().await?;
    info!("Current rate (wei): {}", current_rate);
    let current_rate_decimal = wei_to_decimal_string(current_rate);
    info!("Current rate (decimal): {}", current_rate_decimal);

    // Setup other contracts
    let gaf_address = Address::from_str(GAF_TOKEN_ADDRESS)?;
    let igaf_address = Address::from_str(IGAF_TOKEN_ADDRESS)?;
    let teller_address = Address::from_str(SIMPLIFIED_TELLER_ADDRESS)?;

    let gaf = ERC20::new(gaf_address, provider.clone());
    let igaf = ERC20::new(igaf_address, provider.clone());
    let teller = SimplifiedTeller::new(teller_address, provider.clone());

    let buffer = U256::from(0).pow(U256::from(17)); // 0.1 * 10^18

    let action_amount_control_str = env::var("ACTION_AMOUNT_CONTROL")
        .unwrap_or_else(|_| DEFAULT_ACTION_AMOUNT_CONTROL.to_string());
    let action_amount_control: f64 = action_amount_control_str.parse().unwrap_or(1.0);
    info!("Action amount control: {}", action_amount_control);

    let max_deposit_rate: f64 = env::var("MAX_DEPOSIT_RATE")
        .unwrap_or_else(|_| DEFAULT_MAX_DEPOSIT_RATE.to_string())
        .parse()
        .unwrap_or(DEFAULT_MAX_DEPOSIT_RATE);
    let min_withdraw_rate: f64 = env::var("MIN_WITHDRAW_RATE")
        .unwrap_or_else(|_| DEFAULT_MIN_WITHDRAW_RATE.to_string())
        .parse()
        .unwrap_or(DEFAULT_MIN_WITHDRAW_RATE);
    // Make sure every withdrawal yields some profit
    if min_withdraw_rate < 1.0 {
        panic!("MIN_WITHDRAW_RATE must be >= 1.0");
    }

    info!(
        "MAX_DEPOSIT_RATE:      {}",
        U256::from(max_deposit_rate * WEI_SCALE_F64)
    );
    info!(
        "MIN_WITHDRAW_RATE:     {}",
        U256::from(min_withdraw_rate * WEI_SCALE_F64)
    );
    info!("Current Exchange Rate: {}", current_rate);
    // Execute strategy
    if current_rate > last_action_rate
        && current_rate > U256::from(min_withdraw_rate * WEI_SCALE_F64)
    {
        info!(
            "Current rate > Last action rate && current_rate > {min_withdraw_rate}. Start withdrawing..."
        );
        let igaf_balance = igaf.balanceOf(my_address).call().await?;
        info!("iGAF Balance: {}", igaf_balance);

        if let Some(available_balance) = available_after_buffer(igaf_balance, buffer) {
            let amount_to_withdraw =
                calculate_action_amount(available_balance, action_amount_control);

            if amount_to_withdraw > U256::ZERO {
                info!(
                    "Withdrawing iGAF: {} (Balance: {} - Buffer: {})",
                    amount_to_withdraw, igaf_balance, buffer
                );

                let minimum_assets = calculate_minimum_assets(amount_to_withdraw, current_rate);
                info!("minimum_assets: {minimum_assets}");

                // Simulate the approval transaction
                info!("Simulating iGAF approve transaction...");
                match igaf.approve(teller_address, amount_to_withdraw).call().await {
                    Ok(_) => info!("Approve simulation succeeded"),
                    Err(e) => {
                        info!("Approve simulation failed: {e}. Aborting withdrawal.");
                        let summary = hold_record(current_rate_decimal.clone());
                        info!("Action result: {:?}", summary);
                        return Ok(summary);
                    }
                }

                // Simulate the withdrawal transaction
                info!("Simulating withdraw transaction...");
                match teller
                    .withdraw(gaf_address, amount_to_withdraw, minimum_assets)
                    .call()
                    .await
                {
                    Ok(_) => info!("Withdraw simulation succeeded"),
                    Err(e) => {
                        info!("Withdraw simulation failed: {e}. Aborting withdrawal.");
                        let summary = hold_record(current_rate_decimal.clone());
                        info!("Action result: {:?}", summary);
                        return Ok(summary);
                    }
                }

                // Execute the actual approve transaction
                igaf.approve(teller_address, amount_to_withdraw)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Approved iGAF for teller");

                // Execute the actual withdraw transaction
                let withdraw_tx_hash = teller
                    .withdraw(gaf_address, amount_to_withdraw, minimum_assets)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Withdrawn iGAF: {withdraw_tx_hash:#x}");

                let gaf_amount_u256 = calculate_minimum_assets(amount_to_withdraw, current_rate);
                let gaf_amount = wei_to_decimal_string(gaf_amount_u256);
                let last_withdraw = record::get_last_withdraw_amount()?;
                let amount_diff = if let Some(last) = last_withdraw {
                    let last_wei = decimal_string_to_wei(&last)?;
                    Some(signed_decimal_difference(gaf_amount_u256, last_wei))
                } else {
                    None
                };

                let summary = Record {
                    timestamp: Option::from(Utc::now().to_rfc3339()),
                    action_type: "withdraw".to_string(),
                    gaf_amount: Some(gaf_amount.clone()),
                    current_exchange_rate: current_rate_decimal.clone(),
                    amount_diff,
                    transaction_hash: Some(format!("{withdraw_tx_hash:#x}")),
                };
                info!("Action result: {:?}", summary);
                Ok(summary)
            } else {
                info!("Calculated withdraw amount is 0 or insufficient balance for fixed amount.");
                let summary = hold_record(current_rate_decimal.clone());
                info!("Action result: {:?}", summary);
                Ok(summary)
            }
        } else {
            info!(
                "Not enough iGAF to withdraw (balance {} <= buffer {}).",
                igaf_balance, buffer
            );
            let summary = hold_record(current_rate_decimal.clone());
            info!("Action result: {:?}", summary);
            Ok(summary)
        }
    } else if current_rate < last_action_rate
        && current_rate < U256::from(max_deposit_rate * WEI_SCALE_F64)
    {
        info!(
            "Current rate < Last action rate && current_rate < {max_deposit_rate}. Start depositing..."
        );
        let gaf_balance = gaf.balanceOf(my_address).call().await?;
        info!("GAF Balance: {}", gaf_balance);

        return if let Some(available_balance) = available_after_buffer(gaf_balance, buffer) {
            let amount_to_deposit =
                calculate_action_amount(available_balance, action_amount_control);

            if amount_to_deposit > U256::ZERO {
                info!(
                    "Depositing GAF: {} (Balance: {} - Buffer: {})",
                    amount_to_deposit, gaf_balance, buffer
                );

                let minimum_mint = calculate_minimum_mint(amount_to_deposit, current_rate);
                info!("minimum_mint: {minimum_mint}");

                // Simulate the approval transaction
                info!("Simulating GAF approve transaction...");
                match gaf.approve(igaf_address, amount_to_deposit).call().await {
                    Ok(_) => info!("Approve simulation succeeded"),
                    Err(e) => {
                        info!("Approve simulation failed: {e}. Aborting deposit.");
                        let summary = hold_record(current_rate_decimal.clone());
                        info!("Action result: {:?}", summary);
                        return Ok(summary);
                    }
                }

                // Simulate the deposit transaction
                info!("Simulating deposit transaction...");
                match teller
                    .deposit(gaf_address, amount_to_deposit, minimum_mint)
                    .call()
                    .await
                {
                    Ok(_) => info!("Deposit simulation succeeded"),
                    Err(e) => {
                        info!("Deposit simulation failed: {e}. Aborting deposit.");
                        let summary = hold_record(current_rate_decimal.clone());
                        info!("Action result: {:?}", summary);
                        return Ok(summary);
                    }
                }

                // Execute the actual approve transaction
                gaf.approve(igaf_address, amount_to_deposit)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Approved GAF for teller");

                // Execute the actual deposit transaction
                let deposit_tx_hash = teller
                    .deposit(gaf_address, amount_to_deposit, minimum_mint)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Deposited GAF: {deposit_tx_hash:#x}");

                let gaf_amount = wei_to_decimal_string(amount_to_deposit);

                let summary = Record {
                    timestamp: Option::from(Utc::now().to_rfc3339()),
                    action_type: "deposit".to_string(),
                    gaf_amount: Some(gaf_amount.clone()),
                    current_exchange_rate: current_rate_decimal.clone(),
                    amount_diff: None,
                    transaction_hash: Some(format!("{deposit_tx_hash:#x}")),
                };
                info!("Action result: {:?}", summary);
                Ok(summary)
            } else {
                info!("Calculated deposit amount is 0 or insufficient balance for fixed amount.");
                let summary = hold_record(current_rate_decimal.clone());
                info!("Action result: {:?}", summary);
                Ok(summary)
            }
        } else {
            info!(
                "Not enough GAF to deposit (balance {} <= buffer {}).",
                gaf_balance, buffer
            );
            let summary = hold_record(current_rate_decimal.clone());
            info!("Action result: {:?}", summary);
            Ok(summary)
        };
    } else {
        info!("Rates are equal or no action condition met.");
        let summary = hold_record(current_rate_decimal);
        info!("Action result: {:?}", summary);
        Ok(summary)
    }
}
