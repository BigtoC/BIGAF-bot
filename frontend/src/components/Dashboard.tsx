import React, { useEffect, useMemo, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useAccount, useConnect, useDisconnect } from 'wagmi';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { ModeToggle } from '@/components/mode-toggle';
import { BotChart } from './BotChart';
import { fetchAndParseParquet } from '@/lib/parquet';
import type { BotRecord } from '@/lib/parquet';

const RECORD_URL = 'https://raw.githubusercontent.com/BigtoC/BIGAF-bot/main/record.parquet';
const TWO_HOURS_MS = 2 * 60 * 60 * 1000;

type BotStatusTone = 'active' | 'paused' | 'unknown';

interface StatusInfo {
  lastActionRate?: number;
  lastActionType?: string;
  currentRate?: number;
  latestTimestamp?: number;
  statusLabel: string;
  statusTone: BotStatusTone;
}

const defaultStatus: StatusInfo = {
  lastActionRate: undefined,
  lastActionType: undefined,
  currentRate: undefined,
  latestTimestamp: undefined,
  statusLabel: 'No Data',
  statusTone: 'unknown',
};

const sanitizeRecords = (records?: BotRecord[]): BotRecord[] =>
  Array.isArray(records)
    ? records.filter(
        (record): record is BotRecord => Boolean(record) && Number.isFinite(record.timestamp)
      )
    : [];

const deriveStatusInfo = (records: BotRecord[] | undefined, now: number): StatusInfo => {
  const sanitized = sanitizeRecords(records);

  if (sanitized.length === 0) {
    return { ...defaultStatus };
  }

  const sorted = [...sanitized].sort((a, b) => b.timestamp - a.timestamp);
  const latestRecord = sorted[0];
  const lastActionRecord = sorted.find(
    (record) => record.action_type?.toLowerCase() !== 'hold'
  );

  const lastActionRate = (lastActionRecord ?? latestRecord)?.current_exchange_rate;
  const lastActionType = (lastActionRecord ?? latestRecord)?.action_type;
  const currentRate = latestRecord?.current_exchange_rate;
  const latestTimestamp = latestRecord?.timestamp;

  if (typeof latestTimestamp === 'number' && Number.isFinite(latestTimestamp)) {
    const isFresh = now - latestTimestamp <= TWO_HOURS_MS;
    return {
      lastActionRate,
      lastActionType,
      currentRate,
      latestTimestamp,
      statusLabel: isFresh ? 'Active' : 'Paused',
      statusTone: isFresh ? 'active' : 'paused',
    };
  }

  return {
    lastActionRate,
    lastActionType,
    currentRate,
    latestTimestamp: undefined,
    statusLabel: 'Unknown',
    statusTone: 'unknown',
  };
};

const formatRate = (value?: number) =>
  typeof value === 'number' && Number.isFinite(value) ? value.toFixed(4) : '—';

const formatActionType = (value?: string) => {
  if (!value) return '—';
  return value
    .split(/[\s_-]+/)
    .map((segment) =>
      segment ? segment[0].toUpperCase() + segment.slice(1).toLowerCase() : segment
    )
    .join(' ');
};

const getNow = () => Date.now();

const Dashboard: React.FC = () => {
  const { address, isConnected } = useAccount();
  const { connectors, connect } = useConnect();
  const { disconnect } = useDisconnect();

  const { data: chartData, isLoading, error } = useQuery<BotRecord[]>({
    queryKey: ['bot-records'],
    queryFn: () => fetchAndParseParquet(RECORD_URL),
    retry: 1,
  });

  const [now, setNow] = useState(() => getNow());

  const statusInfo = useMemo(() => deriveStatusInfo(chartData, now), [chartData, now]);

  useEffect(() => {
    const timer = setInterval(() => setNow(getNow()), 60_000);
    return () => clearInterval(timer);
  }, []);

  const statusClass =
    statusInfo.statusTone === 'active'
      ? 'text-green-500'
      : statusInfo.statusTone === 'paused'
        ? 'text-yellow-500'
        : 'text-muted-foreground';

  return (
    <div className="p-8 space-y-8">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">BIGAF Bot Dashboard</h1>
        <div className="flex items-center gap-4">
          <ModeToggle />
          {isConnected ? (
            <div className="flex items-center gap-4">
              <span>{address?.slice(0, 6)}...{address?.slice(-4)}</span>
              <Button variant="outline" onClick={() => disconnect()}>Disconnect</Button>
            </div>
          ) : (
            <div className="flex gap-2">
              {connectors.map((connector) => (
                <Button
                  key={connector.uid}
                  onClick={() => connect({ connector })}
                >
                  Connect {connector.name}
                </Button>
              ))}
            </div>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Current Status</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Last Action Rate:</span>
                <span className="font-mono">{formatRate(statusInfo.lastActionRate)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Last Action Type:</span>
                <span className="font-mono">{formatActionType(statusInfo.lastActionType)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Last Exchange Rate:</span>
                <span className="font-mono">{formatRate(statusInfo.currentRate)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Bot Status:</span>
                <span className={`${statusClass} font-medium`}>{statusInfo.statusLabel}</span>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Wallet Info</CardTitle>
          </CardHeader>
          <CardContent>
             <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-muted-foreground">GAF Balance:</span>
                <span className="font-mono">1000.00</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">iGAF Balance:</span>
                <span className="font-mono">500.00</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {isLoading ? (
        <div className="flex justify-center p-8">Loading chart data...</div>
      ) : error ? (
        <div className="flex justify-center p-8 text-destructive">
          Failed to load chart data. Please refresh to try again.
        </div>
      ) : (
        <BotChart data={chartData || []} />
      )}
    </div>
  );
};

export default Dashboard;
