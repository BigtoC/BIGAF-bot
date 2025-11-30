import { parquetReadObjects } from 'hyparquet';
import { compressors } from 'hyparquet-compressors';

export interface BotRecord {
  timestamp: number;
  current_exchange_rate: number;
  gaf_amount: number;
  action_type: string;
  transaction_hash?: string;
  amount_diff?: string;
  raw_current_exchange_rate: string;
  raw_gaf_amount: string;
}

export async function fetchAndParseParquet(url: string): Promise<BotRecord[]> {
  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch parquet file: ${response.statusText}`);
    }

    const arrayBuffer = await response.arrayBuffer();
    const rows = await parquetReadObjects({
      file: arrayBuffer,
      compressors
    });

    return rows
      .map((row: Record<string, unknown>) => {
        const tsVal = row.timestamp;
        const timestamp = typeof tsVal === 'string'
          ? new Date(tsVal).getTime()
          : typeof tsVal === 'number'
            ? tsVal
            : NaN;

        if (!Number.isFinite(timestamp)) {
          return null;
        }

        return {
          timestamp,
          current_exchange_rate: Number(row.current_exchange_rate ?? 0),
          gaf_amount: Number(row.gaf_amount ?? 0),
          action_type: String(row.action_type ?? ''),
          transaction_hash: row.transaction_hash ? String(row.transaction_hash) : undefined,
          amount_diff: row.amount_diff ? String(row.amount_diff) : undefined,
          raw_current_exchange_rate: String(row.current_exchange_rate ?? ''),
          raw_gaf_amount: String(row.gaf_amount ?? ''),
        } as BotRecord;
      })
      .filter((record): record is BotRecord => record !== null);
  } catch (error) {
    console.error('Error parsing parquet:', error);
    throw error;
  }
}
