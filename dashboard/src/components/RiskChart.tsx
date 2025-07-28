import React from 'react';
import {
  Chart as ChartJS,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';

ChartJS.register(LineElement, CategoryScale, LinearScale, PointElement, Tooltip, Legend);

const mockTimestamps = Array.from({ length: 10 }, (_, i) => Date.now() - i * 1000 * 60).reverse();
const mockData = mockTimestamps.map((ts, i) => ({
  timestamp: ts,
  risk_score: Math.floor(30 + Math.random() * 70),
}));

const RiskChart: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl">
      <h2 className="text-xl font-semibold mb-2">📉 Risk Over Time</h2>
      <Line
        data={{
          labels: mockData.map((d) => new Date(d.timestamp).toLocaleTimeString()),
          datasets: [
            {
              label: 'Risk Score',
              data: mockData.map((d) => d.risk_score),
              fill: false,
              borderColor: 'rgb(255, 99, 132)',
              tension: 0.3,
            },
          ],
        }}
        options={{
          responsive: true,
          plugins: {
            legend: { display: false },
          },
          scales: {
            y: {
              min: 0,
              max: 100,
              ticks: { stepSize: 20 },
              title: { display: true, text: 'Risk Score' },
            },
            x: {
              title: { display: true, text: 'Timestamp' },
            },
          },
        }}
      />
    </div>
  );
};

export default RiskChart;
