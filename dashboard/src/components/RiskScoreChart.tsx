import React from 'react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Tooltip,
} from 'chart.js';

ChartJS.register(LineElement, CategoryScale, LinearScale, PointElement, Tooltip);

interface RiskScoreChartProps {
  scores: number[];
}

const RiskScoreChart: React.FC<RiskScoreChartProps> = ({ scores }) => {
  const data = {
    labels: scores.map((_, i) => `T-${scores.length - i}`),
    datasets: [
      {
        label: 'Risk Score',
        data: scores,
        borderColor: 'rgb(220, 38, 38)',
        backgroundColor: 'rgba(220, 38, 38, 0.2)',
        tension: 0.3,
      },
    ],
  };

  const options = {
    responsive: true,
    scales: {
      y: { beginAtZero: true, max: 1 },
    },
  };

  return <Line data={data} options={options} />;
};

export default RiskScoreChart;
