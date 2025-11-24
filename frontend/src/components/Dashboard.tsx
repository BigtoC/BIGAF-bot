import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { useAccount, useConnect, useDisconnect } from 'wagmi';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { ModeToggle } from '@/components/mode-toggle';
import { BotChart } from './BotChart';
import { fetchAndParseParquet } from '@/lib/parquet';

const RECORD_URL = 'https://raw.githubusercontent.com/BigtoC/BIGAF-bot/main/record.parquet';

const Dashboard: React.FC = () => {
  const { address, isConnected } = useAccount();
  const { connectors, connect } = useConnect();
  const { disconnect } = useDisconnect();

  const { data: chartData, isLoading, error } = useQuery({
    queryKey: ['bot-records'],
    queryFn: () => fetchAndParseParquet(RECORD_URL),
    retry: 1,
  });

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
                <span className="font-mono">1.2345</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Current Rate:</span>
                <span className="font-mono">1.2350</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Bot Status:</span>
                <span className="text-green-500 font-medium">Active</span>
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
