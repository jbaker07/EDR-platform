import React, { useEffect, useState } from 'react';
import { X } from 'lucide-react';
import axios from 'axios';
import RiskScoreChart from './RiskScoreChart';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
} from 'chart.js';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Tooltip, Legend);

interface Process {
  pid: number;
  name: string;
  command: string;
  memory: number;
  cpu_usage: number;
  status?: string;
  severity?: string;
}

interface Props {
  process: Process;
  onClose: () => void;
}

export default function ProcessDetailModal({ process, onClose }: Props) {
  const [risk, setRisk] = useState<number | null>(null);
  const [history, setHistory] = useState<number[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [avgCPU, setAvgCPU] = useState<number>(10.5);
  const [avgMem, setAvgMem] = useState<number>(512 * 1024 * 1024); // 512MB default

  useEffect(() => {
    const fetchRisk = async () => {
      try {
        const res = await axios.post('http://localhost:8000/api/score', [
          {
            pid: process.pid,
            memory: process.memory ?? 0,
            cpu_usage: process.cpu_usage ?? 0,
          },
        ]);
        const score = res.data.results?.[0]?.risk ?? 0.5;
        setRisk(score);
        setHistory([0.2, 0.3, 0.6, 0.8, score]);
      } catch (err) {
        setError('Risk scoring failed');
      } finally {
        setLoading(false);
      }
    };
    fetchRisk();
  }, [process]);

  return (
    <div className="fixed inset-0 z-50 bg-black bg-opacity-40 flex justify-end">
      <div className="w-full max-w-lg h-full bg-white shadow-xl flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-4 border-b bg-gray-100">
          <div>
            <h2 className="text-xl font-bold">{process.name}</h2>
            <p className="text-sm text-gray-500">PID: {process.pid}</p>
          </div>
          <button onClick={onClose}>
            <X className="w-6 h-6 text-gray-500 hover:text-red-500" />
          </button>
        </div>

        {/* Body */}
        <div className="p-6 flex-1 overflow-y-auto">
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-gray-500">Command</p>
              <p className="font-mono text-gray-800">{process.command || 'N/A'}</p>
            </div>
            <div>
              <p className="text-gray-500">CPU Usage</p>
              <p>
                {process.cpu_usage.toFixed(1)}%
                <span className="text-xs text-gray-500"> (avg: {avgCPU}%)</span>
              </p>
            </div>
            <div>
              <p className="text-gray-500">Memory</p>
              <p>
                {(process.memory / 1024 / 1024).toFixed(2)} MB
                <span className="text-xs text-gray-500">
                  {' '}
                  (avg: {(avgMem / 1024 / 1024).toFixed(0)} MB)
                </span>
              </p>
            </div>
            {process.status && (
              <div>
                <p className="text-gray-500">Status</p>
                <p className="capitalize">{process.status}</p>
              </div>
            )}
            <div>
              <p className="text-gray-500">Live Risk Score</p>
              {loading ? (
                <p className="text-blue-500">Calculating...</p>
              ) : error ? (
                <p className="text-red-500">{error}</p>
              ) : (
                <p className="font-bold text-lg text-gray-900">{risk?.toFixed(2)}</p>
              )}
            </div>
            {process.severity && (
              <div>
                <p className="text-gray-500">Severity</p>
                <span
                  className={`inline-block px-2 py-1 text-xs font-semibold rounded ${
                    process.severity === 'high'
                      ? 'bg-red-100 text-red-800'
                      : process.severity === 'medium'
                      ? 'bg-yellow-100 text-yellow-800'
                      : 'bg-green-100 text-green-800'
                  }`}
                >
                  {process.severity.toUpperCase()}
                </span>
              </div>
            )}
          </div>

          {/* Risk Score Chart */}
          <div className="mt-6">
            <h3 className="text-sm font-semibold text-gray-700 mb-2">📈 Risk Score History</h3>
            <RiskScoreChart scores={history} />
          </div>

          {/* Telemetry Trends */}
          <div className="mt-6">
            <h3 className="text-sm font-semibold text-gray-700 mb-2">📊 Telemetry Trends</h3>
            <Line
              data={{
                labels: ['T-6', 'T-5', 'T-4', 'T-3', 'T-2', 'T-1', 'Now'],
                datasets: [
                  {
                    label: 'CPU (%)',
                    data: [10, 30, 40, 20, 60, 50, 70],
                    borderColor: 'rgba(75, 192, 192, 1)',
                    backgroundColor: 'rgba(75, 192, 192, 0.1)',
                    fill: true,
                    tension: 0.3,
                  },
                  {
                    label: 'Memory (MB)',
                    data: [200, 400, 350, 500, 650, 600, 800],
                    borderColor: 'rgba(255, 99, 132, 1)',
                    backgroundColor: 'rgba(255, 99, 132, 0.1)',
                    fill: true,
                    tension: 0.3,
                  },
                ],
              }}
              options={{
                responsive: true,
                plugins: { legend: { position: 'top' } },
                scales: {
                  y: { beginAtZero: true },
                },
              }}
            />
          </div>
        </div>

        {/* Actions */}
        <div className="p-4 border-t bg-gray-50 flex justify-end gap-3">
          <button
            className="bg-yellow-100 text-yellow-800 px-4 py-2 rounded hover:bg-yellow-200 text-sm"
            onClick={() => alert('Tagged for review.')}
          >
            🏷️ Tag
          </button>
          <button
            className="bg-red-100 text-red-800 px-4 py-2 rounded hover:bg-red-200 text-sm"
            onClick={() => alert('Terminating process...')}
          >
            ❌ Terminate
          </button>
          <button
            className="bg-blue-100 text-blue-800 px-4 py-2 rounded hover:bg-blue-200 text-sm"
            onClick={() => alert('Quarantining process...')}
          >
            🛡️ Quarantine
          </button>
        </div>
      </div>
    </div>
  );
}
