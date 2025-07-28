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
import AgentRegistryPanel from '@/components/AgentRegistryPanel';

ChartJS.register(LineElement, CategoryScale, LinearScale, PointElement, Tooltip, Legend);

interface AlertTableProps {
  alerts?: Alert[];
  onRowClick?: (alert: any) => void; // ← Accepts selected process with metadata
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
      const res = await fetch('/api/telemetry/all'); // use relative URL for Vite proxy
      const data = await res.json();

      const normalized: ProcessInfo[] = (data.processes as any[])
      .filter(p => p.pid !== 0 && (p.cpu_percent > 0 || p.memory > 0))  // 💥 Skip empty/default
      .map(p => ({
      pid:        p.pid,
      name:       p.process_name,
      cmd:        p.cmd,
      cpu_usage:  p.cpu_percent ?? 0,
      memory:     p.memory ?? 0,
      risk_score: Math.round(p.cpu_percent ?? 0),
      risk_level:
        (p.cpu_percent ?? 0) > 80
          ? 'high'
          : (p.cpu_percent ?? 0) > 50
            ? 'medium'
            : 'low',
      history:    p.history ?? [],
      endpoint_id:p.endpoint_id,
  }));


      const groupedByHost: Record<string, ProcessInfo[]> = {};
      for (const proc of normalized) {
        const host = proc.endpoint_id || data.hostname || 'unknown-host';
        if (!groupedByHost[host]) groupedByHost[host] = [];
        groupedByHost[host].push(proc);
      }

      const aggregated: Alert[] = Object.entries(groupedByHost).map(
        ([hostname, processes]) => ({
          hostname,
          timestamp: data.timestamp,
          processes,
        })
      );

      setAlerts(aggregated);
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
        .filter(proc => {
          const name = proc.name?.toLowerCase() || '';
          return (filter === 'all' || proc.risk_level === filter) &&
                 name.includes(search.toLowerCase());
        })
        .sort((a, b) => {
          if (sortKey === 'pid') return a.pid - b.pid;
          const levels = { low: 1, medium: 2, high: 3 };
          return (levels[b.risk_level || 'low'] - levels[a.risk_level || 'low']);
        }),
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

      <div className="flex justify-end mb-4 gap-3">
        <button onClick={fetchLiveTelemetry} className="bg-blue-600 text-white px-3 py-1 rounded">
          🔄 Refresh Now
        </button>
        <button
          onClick={() => setLive(!live)}
          className={`px-3 py-1 rounded text-sm ${
            live ? 'bg-green-600 text-white' : 'bg-gray-300 text-black'
          }`}
        >
          {live ? '🔵 Live' : '⚪ Paused'}
        </button>
      </div>

      <div className="flex flex-wrap gap-4 mb-6">
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔢 Total Hosts</p>
          <p className="text-lg font-bold">{alerts.length}</p>
        </div>
        <div className="bg-gray-700 text-white p-4 rounded shadow">
          <p className="text-sm">🔥 High-Risk Procs</p>
          <p className="text-lg font-bold">
            {alerts.flatMap(a => a.processes).filter(p => p.risk_level === 'high').length}
          </p>
        </div>
      </div>

      <div className="flex flex-wrap gap-4 items-center mb-4">
        <label>
          Sort By:
          <select
            className="ml-2 p-1 text-black"
            value={sortKey}
            onChange={e => setSortKey(e.target.value as any)}
          >
            <option value="pid">PID</option>
            <option value="risk_level">Severity</option>
          </select>
        </label>
        <label>
          Filter:
          <select
            className="ml-2 p-1 text-black"
            value={filter}
            onChange={e => setFilter(e.target.value as any)}
          >
            <option value="all">All</option>
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
          </select>
        </label>
        <input
          className="p-1 border border-gray-300 rounded text-black"
          placeholder="Search process name"
          value={search}
          onChange={e => setSearch(e.target.value)}
        />
      </div>

      <div className="overflow-x-auto bg-gray-800 rounded-lg shadow-lg">
        <table className="min-w-full table-auto text-sm text-left text-white">
          <thead className="bg-gray-700 text-xs uppercase">
            <tr>
              <th className="px-4 py-3">Host</th>
              <th className="px-4 py-3">PID</th>
              <th className="px-4 py-3">Name</th>
              <th className="px-4 py-3">CPU%</th>
              <th className="px-4 py-3">Memory</th>
              <th className="px-4 py-3">Severity</th>
            </tr>
          </thead>
          <tbody>
            {getSortedAndFiltered().map((alert, idx) => (
              <React.Fragment key={idx}>
                <tr className="bg-gray-700">
                  <td colSpan={6} className="px-4 py-2 font-bold">
                    🖥️ {alert.hostname} ({alert.processes.length} procs)
                  </td>
                </tr>
                {alert.processes.map((proc, i) => {
                  const procWithHost = {
                    ...proc,
                    hostname: alert.hostname,
                    timestamp: alert.timestamp,
                  };

                  return (
                    <tr
                      key={`${idx}-${i}`}
                      className={`${i % 2 === 0 ? 'bg-gray-900' : 'bg-gray-800'} cursor-pointer`}
                      onClick={() => {
                        setSelectedProcess(proc);
                        if (onRowClick) onRowClick(procWithHost); // 👈 send full metadata
                      }}
                    >
                      <td className="px-4 py-2">{alert.hostname}</td>
                      <td className="px-4 py-2">{proc.pid}</td>
                      <td className="px-4 py-2">{proc.name}</td>
                      <td className="px-4 py-2">{proc.cpu_usage.toFixed(1)}%</td>
                      <td className="px-4 py-2">{(proc.memory / 1024 / 1024).toFixed(1)} MB</td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs font-bold ${
                          proc.risk_level === 'high'
                            ? 'bg-red-600 text-white'
                            : proc.risk_level === 'medium'
                            ? 'bg-yellow-500 text-black'
                            : 'bg-green-500 text-black'
                        }`}>
                          {proc.risk_level?.toUpperCase()}
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
        <ProcessModal
          process={selectedProcess}
          onClose={() => setSelectedProcess(null)}
        />
      )}
    </div>
  );
};

export default AlertTable;
