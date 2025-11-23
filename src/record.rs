use anyhow::Result;
use chrono::Utc;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

use crate::constant::RECORD_FILE;

const ZERO_DECIMAL: &str = "0.000000000000000000";

#[derive(Debug)]
pub struct Record {
    pub timestamp: Option<String>,
    pub action_type: String,
    pub gaf_amount: Option<String>,
    pub current_exchange_rate: String,
    pub amount_diff: Option<String>,
    pub transaction_hash: Option<String>,
}

const RECORD_COLUMNS: [&str; 6] = [
    "timestamp",
    "action_type",
    "gaf_amount",
    "current_exchange_rate",
    "amount_diff",
    "transaction_hash",
];

fn format_f64_to_decimal(value: f64) -> Option<String> {
    value.is_finite().then(|| format!("{value:.18}"))
}

fn read_decimal_string(row: &DataFrame, column_name: &str) -> Result<Option<String>> {
    let series = match row.column(column_name) {
        Ok(series) => series,
        Err(_) => return Ok(None),
    };

    if let Ok(values) = series.str() {
        if let Some(value) = values.get(0) {
            return Ok(Some(value.to_string()));
        }
    }

    if let Ok(values) = series.f64() {
        return Ok(values.get(0).and_then(format_f64_to_decimal));
    }

    Ok(None)
}

fn enforce_schema(df: &mut DataFrame) -> Result<()> {
    let len = df.height();

    for column in RECORD_COLUMNS {
        if df.column(column).is_err() {
            let null_series = Series::new(column.into(), vec![None::<String>; len]);
            df.with_column(null_series)?;
        }
    }

    for column in ["gaf_amount", "current_exchange_rate", "amount_diff"] {
        if let Ok(series) = df.column(column) {
            if series.str().is_ok() {
                continue;
            }

            if let Ok(values) = series.f64() {
                let converted: Vec<Option<String>> = values
                    .into_iter()
                    .map(|value| value.and_then(format_f64_to_decimal))
                    .collect();
                let new_series = Series::new(column.into(), converted);
                df.replace(column, new_series)?;
            }
        }
    }

    *df = df.select(&RECORD_COLUMNS.map(|c| c.to_string()))?;

    Ok(())
}

// Get last non hold record
pub fn get_last_non_hold_record() -> Result<Option<Record>> {
    if !Path::new(RECORD_FILE).exists() {
        return Ok(None);
    }

    let df = LazyFrame::scan_parquet(RECORD_FILE, ScanArgsParquet::default())?
        .filter(col("action_type").neq(lit("hold")))
        .collect()?;

    if df.height() == 0 {
        return Ok(None);
    }

    let last_row = df.tail(Some(1));

    let timestamp = last_row
        .column("timestamp")
        .ok()
        .and_then(|col| col.str().ok()?.get(0).map(|s| s.to_string()));

    let action_type = last_row
        .column("action_type")?
        .str()?
        .get(0)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "hold".to_string());

    let gaf_amount = read_decimal_string(&last_row, "gaf_amount")?;

    let current_exchange_rate = read_decimal_string(&last_row, "current_exchange_rate")?
        .unwrap_or_else(|| ZERO_DECIMAL.to_string());

    let amount_diff = read_decimal_string(&last_row, "amount_diff")?;

    let transaction_hash = last_row
        .column("transaction_hash")
        .ok()
        .and_then(|col| col.str().ok()?.get(0).map(|s| s.to_string()));

    Ok(Some(Record {
        timestamp,
        action_type,
        gaf_amount,
        current_exchange_rate,
        amount_diff,
        transaction_hash,
    }))
}

pub fn get_last_withdraw_amount() -> Result<Option<String>> {
    if !Path::new(RECORD_FILE).exists() {
        return Ok(None);
    }

    let df = LazyFrame::scan_parquet(RECORD_FILE, ScanArgsParquet::default())?
        .filter(col("action_type").eq(lit("withdraw")))
        .collect()?;

    if df.height() == 0 {
        return Ok(None);
    }

    let last_row = df.tail(Some(1));
    let amount = read_decimal_string(&last_row, "gaf_amount")?;
    Ok(amount)
}

pub fn append_record(record: Record) -> Result<()> {
    let Record {
        timestamp,
        action_type,
        gaf_amount,
        current_exchange_rate,
        amount_diff,
        transaction_hash,
    } = record;

    let timestamp = timestamp.unwrap_or_else(|| Utc::now().to_rfc3339());

    let mut new_df = polars::df![
        "timestamp" => &[timestamp.as_str()],
        "action_type" => &[action_type.as_str()],
        "gaf_amount" => &[gaf_amount.as_deref()],
        "current_exchange_rate" => &[current_exchange_rate.as_str()],
        "amount_diff" => &[amount_diff.as_deref()],
        "transaction_hash" => &[transaction_hash.as_deref()],
    ]?;
    enforce_schema(&mut new_df)?;

    if Path::new(RECORD_FILE).exists() {
        let mut existing_df =
            LazyFrame::scan_parquet(RECORD_FILE, ScanArgsParquet::default())?.collect()?;
        enforce_schema(&mut existing_df)?;
        let mut final_df = existing_df.vstack(&new_df)?;
        let file = File::create(RECORD_FILE)?;
        ParquetWriter::new(file).finish(&mut final_df)?;
    } else {
        let file = File::create(RECORD_FILE)?;
        ParquetWriter::new(file).finish(&mut new_df)?;
    }

    Ok(())
}
