import React, { useEffect, useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Bar } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Tooltip,
  Legend,
} from 'chart.js';

ChartJS.register(CategoryScale, LinearScale, BarElement, Tooltip, Legend);

interface RiskEntry {
  hostname: string;
  trust: number;
}

export default function GlobalRiskHeatmap() {
  const [data, setData] = useState<RiskEntry[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await fetch('/api/trust/global');
        const json = await res.json();
        if (Array.isArray(json)) {
          setData(json);
        } else {
          console.warn('Unexpected global trust data format:', json);
          setData([]);
        }
      } catch (err) {
        console.error('Failed to load global trust data', err);
      }
    };

    fetchData();
    const id = setInterval(fetchData, 10000);
    return () => clearInterval(id);
  }, []);

  return (
    <Card className="mb-6">
      <CardContent>
        <h2 className="text-lg font-semibold mb-4">🌐 Global Risk Heatmap</h2>
        {data.length > 0 ? (
          <Bar
            data={{
              labels: data.map((d) => d.hostname),
              datasets: [
                {
                  label: 'Trust Score',
                  data: data.map((d) => d.trust),
                  backgroundColor: data.map((d) =>
                    d.trust < 30
                      ? 'rgba(220, 38, 38, 0.7)' // red
                      : d.trust < 60
                      ? 'rgba(234, 179, 8, 0.7)' // yellow
                      : 'rgba(34, 197, 94, 0.7)' // green
                  ),
                },
              ],
            }}
            options={{
              responsive: true,
              plugins: {
                legend: { display: false },
              },
              scales: {
                y: { beginAtZero: true, max: 100 },
              },
            }}
          />
        ) : (
          <p className="text-sm text-gray-500">No trust data available.</p>
        )}
      </CardContent>
    </Card>
  );
}

