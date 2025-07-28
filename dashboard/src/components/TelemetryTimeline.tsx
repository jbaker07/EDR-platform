// src/components/TelemetryTimeline.tsx
import React from 'react';
import {
  Chart as ChartJS,
  TimeScale,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
  ChartOptions,
} from 'chart.js';
import 'chartjs-adapter-date-fns';              // ← important: pull in the date adapter
import { Line } from 'react-chartjs-2';
import { Alert } from '@/types/Alert';

ChartJS.register(
  TimeScale,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend
);

interface TelemetryTimelineProps {
  alerts?: Alert[];   // optional, default []
}

export default function TelemetryTimeline({
  alerts = [],
}: TelemetryTimelineProps) {
  // build an array of { x: timestamp, y: risk_score }
  const points = alerts.flatMap(alert =>
    alert.processes.flatMap(proc =>
      (proc.history || []).map(h => ({
        x: h.timestamp * 1000,   // Chart.js expects ms
        y: h.risk_score,
      }))
    )
  );

  // nothing to show yet
  if (points.length === 0) {
    return (
      <div className="p-4 text-gray-500">
        No timeline data yet
      </div>
    );
  }

  const data = {
    datasets: [
      {
        label: 'Risk Score',
        data: points,
        fill: false,
        tension: 0.2,
        borderColor: 'rgba(75,192,192,1)',
        pointRadius: 2,
      },
    ],
  };

  const options: ChartOptions<'line'> = {
    responsive: true,
    plugins: {
      legend: { display: false },
    },
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'minute',
          tooltipFormat: 'p',
        },
        title: { display: true, text: 'Time' },
      },
      y: {
        beginAtZero: true,
        title: { display: true, text: 'Risk Score' },
      },
    },
  };

  return (
    <div className="mb-6">
      <h2 className="text-xl font-semibold mb-2">Telemetry Timeline</h2>
      <Line
        id="telemetry-chart"                   // unique canvas ID
        key={JSON.stringify(points)}           // force remount when data changes
        data={data}
        options={options}
        redraw
      />
    </div>
  );
}
