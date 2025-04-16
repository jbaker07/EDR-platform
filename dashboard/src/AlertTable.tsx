import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
  Tooltip,
  Legend,
} from 'chart.js';

ChartJS.register(LineElement, CategoryScale, LinearScale, PointElement, Tooltip, Legend);

interface ProcessInfo {
  pid: number;
  name: string;
  cmd: string;
  risk_score?: number;
  risk_level?: 'low' | 'medium' | 'high';
  history?: { timestamp: number; risk_score: number }[];
}

interface Alert {
  hostname: string;
  timestamp: number;
  processes: ProcessInfo[];
}

const AlertTable: React.FC = () => {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [sortKey, setSortKey] = useState<'pid' | 'risk_level'>('pid');
  const [filter, setFilter] = useState<'all' | 'low' | 'medium' | 'high'>('all');
  const [search, setSearch] = useState('');
  const [selectedProcess, setSelectedProcess] = useState<ProcessInfo | null>(null);

  const fetchAlerts = async () => {
    try {
      const res = await fetch('http://127.0.0.1:8000/api/telemetry');
      const data = await res.json();
      if (data?.hostname && data?.processes) {
        setAlerts((prev) => [data, ...prev].slice(0, 20));
      }
    } catch (err) {
      console.error('Failed to fetch alerts', err);
    }
  };

  useEffect(() => {
    fetchAlerts();
    const interval = setInterval(fetchAlerts, 5000);
    return () => clearInterval(interval);
  }, []);

  const getSortedAndFiltered = (): Alert[] => {
    return alerts.map(alert => ({
      ...alert,
      processes: alert.processes
        .filter(proc =>
          (filter === 'all' || proc.risk_level === filter) &&
          proc.name.toLowerCase().includes(search.toLowerCase())
        )
        .sort((a, b) => {
          if (sortKey === 'pid') return a.pid - b.pid;
          const levels = { low: 1, medium: 2, high: 3 };
          return (levels[b.risk_level || 'low'] - levels[a.risk_level || 'low']);
        })
    }));
  };

  const renderRiskChart = (history: { timestamp: number; risk_score: number }[]) => (
    <Line
      data={{
        labels: history.map(p => new Date(p.timestamp * 1000).toLocaleTimeString()),
        datasets: [
          {
            label: 'Risk Score',
            data: history.map(p => p.risk_score),
            fill: false,
            borderColor: 'rgba(255,99,132,1)',
            tension: 0.2,
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
    <div className="p-6">
      <h2 className="text-2xl font-semibold mb-4">🚨 Live Endpoint Alerts</h2>

      {/* 🧭 Dashboard Summary */}
      <div className="flex flex-wrap gap-4 mb-6">
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔢 Total Alerts</p>
          <p className="text-lg font-bold">{alerts.length}</p>
        </div>
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🖥️ Endpoints</p>
          <p className="text-lg font-bold">
            {[...new Set(alerts.map(a => a.hostname))].length}
          </p>
        </div>
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔥 High Risk</p>
          <p className="text-lg font-bold">
            {alerts.flatMap(a => a.processes).filter(p => p.risk_level === 'high').length}
          </p>
        </div>
      </div>

      {/* 🧩 Filter & Sort Controls */}
      <div className="flex flex-wrap gap-4 mb-4 items-center">
        <label>
          Sort By:
          <select
            className="ml-2 p-1 text-black"
            value={sortKey}
            onChange={(e) => setSortKey(e.target.value as 'pid' | 'risk_level')}
          >
            <option value="pid">PID</option>
            <option value="risk_level">Severity</option>
          </select>
        </label>

        <label>
          Filter Severity:
          <select
            className="ml-2 p-1 text-black"
            value={filter}
            onChange={(e) => setFilter(e.target.value as 'all' | 'low' | 'medium' | 'high')}
          >
            <option value="all">All</option>
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
          </select>
        </label>

        <input
          type="text"
          className="p-1 text-black border border-gray-300 rounded"
          placeholder="Search by process name"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      {/* 🧱 Grouped Table */}
      <div className="overflow-x-auto bg-gray-800 rounded-lg shadow-lg">
        <table className="min-w-full table-auto text-sm text-left text-white">
          <thead className="bg-gray-700 text-xs uppercase">
            <tr>
              <th className="px-4 py-3">Hostname</th>
              <th className="px-4 py-3">Timestamp</th>
              <th className="px-4 py-3">PID</th>
              <th className="px-4 py-3">Process Name</th>
              <th className="px-4 py-3">Command</th>
              <th className="px-4 py-3">Severity</th>
            </tr>
          </thead>
          <tbody>
            {getSortedAndFiltered().map((alert, idx) => (
              <React.Fragment key={idx}>
                <tr className="bg-gray-600">
                  <td className="px-4 py-2 font-bold text-white" colSpan={6}>
                    🖥️ Endpoint: {alert.hostname} • {alert.processes.length} processes
                  </td>
                </tr>
                {alert.processes.map((proc, i) => (
                  <tr
                    key={`${idx}-${i}`}
                    className={i % 2 === 0 ? 'bg-gray-900 cursor-pointer' : 'bg-gray-800 cursor-pointer'}
                    onClick={() => setSelectedProcess(proc)}
                  >
                    <td className="px-4 py-2">{alert.hostname}</td>
                    <td className="px-4 py-2">{new Date(alert.timestamp * 1000).toLocaleString()}</td>
                    <td className="px-4 py-2">{proc.pid}</td>
                    <td className="px-4 py-2">{proc.name}</td>
                    <td className="px-4 py-2 truncate max-w-md">{proc.cmd}</td>
                    <td className="px-4 py-2">
                      <span
                        className={`inline-block px-2 py-1 rounded text-xs font-bold ${
                          proc.risk_level === 'high'
                            ? 'bg-red-600 text-white'
                            : proc.risk_level === 'medium'
                            ? 'bg-yellow-500 text-black'
                            : proc.risk_level === 'low'
                            ? 'bg-green-500 text-black'
                            : 'bg-gray-500 text-white'
                        }`}
                      >
                        {proc.risk_level?.toUpperCase() || 'UNKNOWN'}
                      </span>
                    </td>
                  </tr>
                ))}
              </React.Fragment>
            ))}
          </tbody>
        </table>
      </div>

      {/* 📈 Modal with Risk Chart */}
      {selectedProcess && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white text-black rounded-lg p-6 w-96 shadow-xl max-h-[90vh] overflow-y-auto">
            <h3 className="text-xl font-bold mb-4">Process Details</h3>
            <p><strong>PID:</strong> {selectedProcess.pid}</p>
            <p><strong>Name:</strong> {selectedProcess.name}</p>
            <p><strong>Command:</strong> {selectedProcess.cmd}</p>
            <p><strong>Risk Score:</strong> {selectedProcess.risk_score ?? 'N/A'}</p>
            <p><strong>Risk Level:</strong> {selectedProcess.risk_level ?? 'Unknown'}</p>

            {selectedProcess.history && (
              <div className="mt-4">
                <p className="text-sm mb-2 font-semibold">📉 Risk Score Trend</p>
                {renderRiskChart(selectedProcess.history)}
              </div>
            )}

            <div className="mt-4 flex gap-3">
              <button
                className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
                onClick={() => alert(`Simulated kill signal sent to PID ${selectedProcess?.pid}`)}
              >
                🛑 Kill Process
              </button>
              <button
                className="px-4 py-2 bg-yellow-500 text-black rounded hover:bg-yellow-600"
                onClick={() => alert(`Simulated quarantine issued for PID ${selectedProcess?.pid}`)}
              >
                🚧 Quarantine
              </button>
              <button
                className="px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                onClick={() => alert(`Log extraction started for PID ${selectedProcess?.pid}`)}
              >
                📄 Extract Logs
              </button>
            </div>

            <button
              className="mt-6 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              onClick={() => setSelectedProcess(null)}
            >
              Close
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default AlertTable;
