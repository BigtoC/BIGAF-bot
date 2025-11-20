use crate::constant::{
    ACCOUNTANT_WITH_RATE_PROVIDERS_ADDRESS, DEFAULT_ACTION_AMOUNT_CONTROL, GAF_TOKEN_ADDRESS,
    IGAF_TOKEN_ADDRESS, LAST_ACTION_RATE_FILE, SIMPLIFIED_TELLER_ADDRESS,
};
use crate::contracts::{
    accountant_with_rate_providers::AccountantWithRateProviders, erc20::ERC20,
    simplified_teller::SimplifiedTeller,
};
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use std::env;
use std::fs;
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

fn update_last_action_rate(current_rate: U256) -> Result<()> {
    fs::write(LAST_ACTION_RATE_FILE, current_rate.to_string())?;
    Ok(())
}

pub async fn execute_strategy(rpc_url: &str, private_key: &str) -> Result<()> {
    // Setup provider
    let signer: PrivateKeySigner = private_key.parse()?;
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(Url::parse(rpc_url)?);

    let my_address = signer.address();
    info!("Running bot with address: {}", my_address);

    // Read last action rate
    let last_action_rate_str =
        fs::read_to_string(LAST_ACTION_RATE_FILE).unwrap_or_else(|_| "0".to_string());
    let last_action_rate = U256::from_str(last_action_rate_str.trim()).unwrap_or(U256::ZERO);
    info!("Last action rate: {}", last_action_rate);

    // Get current rate
    let accountant_address = Address::from_str(ACCOUNTANT_WITH_RATE_PROVIDERS_ADDRESS)?;
    let accountant = AccountantWithRateProviders::new(accountant_address, provider.clone());
    let current_rate = accountant.getRate().call().await?;
    info!("Current rate: {}", current_rate);

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

    if current_rate > last_action_rate {
        info!("Current rate > Last action rate. Start withdrawing...");
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

                // Approve iGAF to Teller
                let tx = igaf
                    .approve(teller_address, amount_to_withdraw)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Approved iGAF: {:?}", tx);

                // Withdraw
                // minimumAssets = shareAmount * rate / 1e18
                let minimum_assets = calculate_minimum_assets(amount_to_withdraw, current_rate);
                info!("minimum_assets: {minimum_assets}");

                let tx = teller
                    .withdraw(gaf_address, amount_to_withdraw, minimum_assets)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Withdrawn iGAF: {:?}", tx);

                // Update last action rate
                update_last_action_rate(current_rate)?;
                info!("Updated last_action_rate.txt to {}", current_rate);
            } else {
                info!("Calculated withdraw amount is 0 or insufficient balance for fixed amount.");
            }
        } else {
            info!(
                "Not enough iGAF to withdraw (balance {} <= buffer {}).",
                igaf_balance, buffer
            );
        }
    } else if current_rate < last_action_rate {
        info!("Current rate < Last action rate. Start depositing...");
        let gaf_balance = gaf.balanceOf(my_address).call().await?;
        info!("GAF Balance: {}", gaf_balance);

        if let Some(available_balance) = available_after_buffer(gaf_balance, buffer) {
            let amount_to_deposit =
                calculate_action_amount(available_balance, action_amount_control);

            if amount_to_deposit > U256::ZERO {
                info!(
                    "Depositing GAF: {} (Balance: {} - Buffer: {})",
                    amount_to_deposit, gaf_balance, buffer
                );

                // Approve GAF to Teller
                let tx = gaf
                    .approve(igaf_address, amount_to_deposit)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Approved GAF: {:?}", tx);

                // Deposit
                // minimumMint = depositAmount * 1e18 / rate
                let minimum_mint = calculate_minimum_mint(amount_to_deposit, current_rate);
                info!("minimum_mint: {minimum_mint}");

                let tx = teller
                    .deposit(gaf_address, amount_to_deposit, minimum_mint)
                    .send()
                    .await?
                    .watch()
                    .await?;
                info!("Deposited GAF: {:?}", tx);

                // Update last action rate
                update_last_action_rate(current_rate)?;
                info!("Updated last_action_rate.txt to {}", current_rate);
            } else {
                info!("Calculated deposit amount is 0 or insufficient balance for fixed amount.");
            }
        } else {
            info!(
                "Not enough GAF to deposit (balance {} <= buffer {}).",
                gaf_balance, buffer
            );
        }
    } else {
        info!("Rates are equal or no action condition met.");
    }

    Ok(())
}
