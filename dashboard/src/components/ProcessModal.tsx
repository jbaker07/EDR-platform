import React from 'react';
import { Line } from 'react-chartjs-2';

type ProcessInfo = {
  pid: number;
  name: string;
  cmd: string;
  risk_score?: number;
  risk_level?: 'low' | 'medium' | 'high';
  history?: { timestamp: number; risk_score: number }[];
};

type ProcessModalProps = {
  process: ProcessInfo | null;
  onClose: () => void;
};

const ProcessModal: React.FC<ProcessModalProps> = ({ process, onClose }) => {
  if (!process) return null;

  const renderRiskChart = (history: { timestamp: number; risk_score: number }[]) => (
    <Line
      data={{
        labels: history.map((h) =>
          new Date(h.timestamp * 1000).toLocaleTimeString()
        ),
        datasets: [
          {
            label: 'Risk Score',
            data: history.map((h) => h.risk_score),
            fill: false,
            borderColor: 'rgba(255, 99, 132, 1)',
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
          y: { min: 0, max: 100 },
        },
      }}
    />
  );

  return (
    <div className="fixed inset-0 z-50 bg-black bg-opacity-50 flex items-center justify-center">
      <div className="bg-white p-6 rounded-lg w-[400px] max-h-[90vh] overflow-y-auto shadow-xl">
        <h2 className="text-xl font-bold mb-4">🧠 Process Details</h2>
        <p><strong>PID:</strong> {process.pid}</p>
        <p><strong>Name:</strong> {process.name}</p>
        <p><strong>Command:</strong> {process.cmd}</p>
        <p><strong>Risk Score:</strong> {process.risk_score ?? 'N/A'}</p>
        <p><strong>Risk Level:</strong> {process.risk_level?.toUpperCase() ?? 'Unknown'}</p>

        {process.history && (
          <div className="mt-4">
            <p className="font-semibold mb-2">📈 Risk Trend</p>
            {renderRiskChart(process.history)}
          </div>
        )}

        <div className="mt-4 flex gap-3 flex-wrap">
          <button
            className="px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700"
            onClick={() => alert(`Kill sent to PID ${process.pid}`)}
          >
            🛑 Kill
          </button>
          <button
            className="px-3 py-2 bg-yellow-400 text-black rounded hover:bg-yellow-500"
            onClick={() => alert(`Quarantine started for PID ${process.pid}`)}
          >
            🚧 Quarantine
          </button>
          <button
            className="px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            onClick={() => alert(`Log extraction for PID ${process.pid}`)}
          >
            📄 Extract Logs
          </button>
        </div>

        <button
          className="mt-6 w-full px-4 py-2 bg-gray-800 text-white rounded hover:bg-gray-900"
          onClick={onClose}
        >
          Close
        </button>
      </div>
    </div>
  );
};

export default ProcessModal;
