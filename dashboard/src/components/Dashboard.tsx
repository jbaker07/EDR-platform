import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import ProcessModal from './ProcessModal';
import { Alert, ProcessInfo } from '@/types/Alert';
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

interface AlertTableProps {
  alerts?: Alert[];
  onRowClick?: (alert: Alert) => void;
}

const AlertTable: React.FC<AlertTableProps> = ({ alerts: externalAlerts = [], onRowClick }) => {
  const [alerts, setAlerts] = useState<Alert[]>(externalAlerts);
  const [sortKey, setSortKey] = useState<'pid' | 'risk_level'>('pid');
  const [filter, setFilter] = useState<'all' | 'low' | 'medium' | 'high'>('all');
  const [search, setSearch] = useState('');
  const [selectedProcess, setSelectedProcess] = useState<ProcessInfo | null>(null);
  const [live, setLive] = useState(true);

  const fetchLiveTelemetry = async () => {
    try {
      const res = await fetch('http://localhost:8000/api/telemetry/all');
      const data = await res.json();
      setAlerts([{ hostname: data.hostname, timestamp: data.timestamp, processes: data.processes }]);
    } catch (err) {
      console.error('Failed to fetch telemetry:', err);
    }
  };

  useEffect(() => {
    fetchLiveTelemetry();
    let interval: NodeJS.Timeout;
    if (live) {
      interval = setInterval(fetchLiveTelemetry, 5000);
    }
    return () => clearInterval(interval);
  }, [live]);

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
          return levels[b.risk_level || 'low'] - levels[a.risk_level || 'low'];
        })
    }));
  };

  const renderRiskChart = (history: { timestamp: number; risk_score: number }[]) => (
    <Line
      data={{
        labels: history.map(p => new Date(p.timestamp * 1000).toLocaleTimeString()),
        datasets: [{
          label: 'Risk Score',
          data: history.map(p => p.risk_score),
          fill: false,
          borderColor: 'rgba(255,99,132,1)',
          tension: 0.2,
        }],
      }}
      options={{
        responsive: true,
        plugins: { legend: { display: false } },
        scales: { y: { min: 0, max: 100 } },
      }}
    />
  );

  return (
    <div className="p-6">
      <h2 className="text-2xl font-semibold mb-4">🚨 Live Endpoint Alerts</h2>

      <div className="flex justify-end items-center gap-3 mb-4">
        <button onClick={fetchLiveTelemetry} className="bg-blue-600 text-white px-3 py-1 text-sm rounded hover:bg-blue-700">
          🔄 Refresh Now
        </button>
        <button
          onClick={() => setLive(!live)}
          className={`text-sm px-3 py-1 rounded ${live ? 'bg-green-600 text-white hover:bg-green-700' : 'bg-gray-300 text-black hover:bg-gray-400'}`}
        >
          {live ? '🔵 Live Polling' : '⚪ Enable Polling'}
        </button>
      </div>

      <div className="flex flex-wrap gap-4 mb-6">
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔢 Total Alerts</p>
          <p className="text-lg font-bold">{alerts.length}</p>
        </div>
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🖥️ Endpoints</p>
          <p className="text-lg font-bold">{[...new Set(alerts.map(a => a.hostname))].length}</p>
        </div>
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔥 High Risk</p>
          <p className="text-lg font-bold">
            {alerts.flatMap(a => a.processes).filter(p => p.risk_level === 'high').length}
          </p>
        </div>
      </div>

      <div className="flex flex-wrap gap-4 mb-4 items-center">
        <label>
          Sort By:
          <select className="ml-2 p-1 text-black" value={sortKey} onChange={e => setSortKey(e.target.value as 'pid' | 'risk_level')}>
            <option value="pid">PID</option>
            <option value="risk_level">Severity</option>
          </select>
        </label>

        <label>
          Filter Severity:
          <select className="ml-2 p-1 text-black" value={filter} onChange={e => setFilter(e.target.value as 'all' | 'low' | 'medium' | 'high')}>
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
          onChange={e => setSearch(e.target.value)}
        />
      </div>

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
                {alert.processes.map((proc, i) => {
                  const isAnomalous = (proc.cpu_usage && proc.cpu_usage > 80) ||
                    (proc.memory && proc.memory > 800 * 1024 * 1024);

                  return (
                    <tr
                      key={`${idx}-${i}`}
                      className={`
                        ${i % 2 === 0 ? 'bg-gray-900' : 'bg-gray-800'}
                        cursor-pointer
                        transition duration-300 ease-in-out
                        ${proc.risk_score && proc.risk_score > 80 ? 'animate-pulse bg-red-950' : ''}`}
                      onClick={() => setSelectedProcess(proc)}
                    >
                      <td className="px-4 py-2">{alert.hostname}</td>
                      <td className="px-4 py-2">{new Date(alert.timestamp * 1000).toLocaleString()}</td>
                      <td className="px-4 py-2">{proc.pid}</td>
                      <td className="px-4 py-2">
                        {proc.name}
                        {isAnomalous && (
                          <span className="ml-2 inline-block bg-pink-500 text-white text-xs px-2 py-0.5 rounded-full animate-pulse">
                            Anomaly
                          </span>
                        )}
                      </td>
                      <td className="px-4 py-2 truncate max-w-md">{proc.cmd}</td>
                      <td className="px-4 py-2">
                        <span className={`inline-block px-2 py-1 rounded text-xs font-bold ${
                          proc.risk_level === 'high'
                            ? 'bg-red-600 text-white'
                            : proc.risk_level === 'medium'
                            ? 'bg-yellow-500 text-black'
                            : proc.risk_level === 'low'
                            ? 'bg-green-500 text-black'
                            : 'bg-gray-500 text-white'
                        }`}>
                          {proc.risk_level?.toUpperCase() || 'UNKNOWN'}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </React.Fragment>
            ))}
          </tbody>
        </table>
      </div>

      {selectedProcess && (
        <ProcessModal process={selectedProcess} onClose={() => setSelectedProcess(null)} />
      )}
    </div>
  );
};

export default AlertTable;
