import { useMemo, useState } from 'react';
import ReactECharts from 'echarts-for-react';
import type { BotRecord } from '@/lib/parquet';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

interface BotChartProps {
  data: BotRecord[];
}

type TimeRange = '24H' | '7D' | '30D' | 'ALL';

export function BotChart({ data }: BotChartProps) {
  const [timeRange, setTimeRange] = useState<TimeRange>('ALL');

  const sanitizedData = useMemo(
    () => data.filter((item): item is BotRecord => Boolean(item) && Number.isFinite(item.timestamp)),
    [data]
  );

  const filteredData = useMemo(() => {
    if (timeRange === 'ALL') return sanitizedData;

    if (sanitizedData.length === 0) return [];
    
    const latestTs = Math.max(...sanitizedData.map(d => d.timestamp));
    const rangeMs = {
      '24H': 24 * 60 * 60 * 1000,
      '7D': 7 * 24 * 60 * 60 * 1000,
      '30D': 30 * 24 * 60 * 60 * 1000,
      'ALL': 0
    }[timeRange];

    const cutoff = latestTs - rangeMs;
    return sanitizedData.filter(d => d.timestamp >= cutoff);
  }, [sanitizedData, timeRange]);

  const option = useMemo(() => {
    const sortedData = [...filteredData].sort((a, b) => a.timestamp - b.timestamp);

    const exchangeColor = '#ff97d5';
    const gafColor = '#FF8C00';

    const rateData = sortedData
      .filter(d => Number.isFinite(d.timestamp) && Number.isFinite(d.current_exchange_rate))
      .map(d => ({
        value: [d.timestamp, d.current_exchange_rate],
        symbol: 'circle',
        symbolSize: 6,
        itemStyle: {
          color: exchangeColor,
          borderColor: '#ffffff',
          borderWidth: 2
        }
      }));

    let lastGafValue = 0;
    const amountData = sortedData.map(d => {
      const isAction = d.action_type === 'deposit' || d.action_type === 'withdraw';
      const pointColor = d.action_type === 'deposit' ? 'red' : 'black';

      if (isAction) {
        lastGafValue = d.gaf_amount;
      }

      return {
        value: [d.timestamp, lastGafValue],
        symbol: 'circle',
        symbolSize: isAction ? 8 : 0,
        itemStyle: {
          color: isAction ? pointColor : gafColor,
          borderColor: '#ffffff',
          borderWidth: 2
        },
        tooltip: {
          formatter: () => `${d.action_type}: ${d.gaf_amount}`
        }
      };
    });

    return {
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross'
        }
      },
      legend: {
        data: ['Exchange Rate', 'GAF Amount']
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'time',
        boundaryGap: false
      },
      yAxis: [
        {
          type: 'value',
          name: 'Exchange Rate',
          scale: true,
          position: 'left',
          axisLine: {
            show: true,
            lineStyle: {
              color: exchangeColor
            }
          }
        },
        {
          type: 'value',
          name: 'GAF Amount',
          scale: true,
          position: 'right',
          axisLine: {
            show: true,
            lineStyle: {
              color: gafColor
            }
          },
          splitLine: { show: false }
        }
      ],
      series: [
        {
          name: 'Exchange Rate',
          type: 'line',
          data: rateData,
          showSymbol: true,
          yAxisIndex: 0,
          areaStyle: {
            color: 'rgba(255, 151, 213, 0.25)'
          },
          lineStyle: {
            color: exchangeColor,
            width: 3
          }
        },
        {
          name: 'GAF Amount',
          type: 'line',
          data: amountData,
          showSymbol: true,
          yAxisIndex: 1,
          areaStyle: {
            color: 'rgba(255, 140, 0, 0.25)'
          },
          lineStyle: {
            color: gafColor,
            width: 3
          }
        }
      ]
    };
  }, [filteredData]);

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Bot Performance</CardTitle>
          <div className="flex gap-2">
            {(['24H', '7D', '30D', 'ALL'] as TimeRange[]).map((range) => (
              <Button
                key={range}
                variant={timeRange === range ? "default" : "outline"}
                size="sm"
                onClick={() => setTimeRange(range)}
              >
                {range}
              </Button>
            ))}
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <ReactECharts option={option} style={{ height: '400px' }} />
      </CardContent>
    </Card>
  );
}
