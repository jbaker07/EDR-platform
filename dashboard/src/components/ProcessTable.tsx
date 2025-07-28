import React, { useEffect, useState } from 'react';
import Fuse from 'fuse.js';
import axios from 'axios';
import ProcessDetailModal from './ProcessDetailModal';

interface ProcessEntry {
  agent_id: string;
  pid: number;
  name: string;
  command: string;
  severity?: string;
  timestamp?: string | number;
  memory?: number;
  cpu_usage?: number;
}

interface ProcessTableProps {
  onAlertSelect?: (id: string) => void;
}

const getBadgeColor = (severity?: string) => {
  const sev = (severity ?? 'low').toLowerCase();
  switch (sev) {
    case 'high':
      return 'bg-red-100 text-red-800';
    case 'medium':
      return 'bg-yellow-100 text-yellow-800';
    case 'low':
    default:
      return 'bg-green-100 text-green-800';
  }
};

const ProcessTable: React.FC<ProcessTableProps> = ({ onAlertSelect }) => {
  const [data, setData] = useState<ProcessEntry[]>([]);
  const [query, setQuery] = useState('');
  const [filtered, setFiltered] = useState<ProcessEntry[]>([]);
  const [selectedProcess, setSelectedProcess] = useState<ProcessEntry | null>(null);
  const [live, setLive] = useState(false);

  const fetchData = async () => {
    try {
      const res = await axios.get('http://localhost:8000/api/telemetry/all');
      const processes = res.data?.processes ?? [];
      setData(processes);
      setFiltered(processes);
    } catch (err) {
      console.error('Failed to fetch telemetry:', err);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  useEffect(() => {
    if (live) {
      const interval = setInterval(() => fetchData(), 10000);
      return () => clearInterval(interval);
    }
  }, [live]);

  useEffect(() => {
    const fuse = new Fuse(data, {
      keys: ['name', 'command', 'agent_id', 'severity'],
      threshold: 0.3,
    });

    if (query.trim()) {
      const result = fuse.search(query).map(r => r.item);
      setFiltered(result);
    } else {
      setFiltered(data);
    }
  }, [query, data]);

  return (
    <div className="bg-white shadow rounded-xl p-6 mb-10">
      <div className="flex justify-between items-center mb-4">
        <input
          type="text"
          placeholder="Search processes..."
          className="w-full max-w-sm border p-2 rounded"
          value={query}
          onChange={e => setQuery(e.target.value)}
        />
        <label className="flex items-center space-x-2 text-sm ml-4">
          <input
            type="checkbox"
            checked={live}
            onChange={() => setLive(prev => !prev)}
            className="accent-blue-600"
          />
          <span>Live Updates</span>
        </label>
      </div>

      <table className="w-full text-sm">
        <thead>
          <tr className="text-left border-b">
            <th className="p-2">Agent</th>
            <th className="p-2">Process Name</th>
            <th className="p-2">PID</th>
            <th className="p-2">Command</th>
            <th className="p-2">Severity</th>
            <th className="p-2">Timestamp</th>
          </tr>
        </thead>
        <tbody>
          {filtered.map((entry, i) => (
            <tr
              key={i}
              className="border-b hover:bg-gray-50 cursor-pointer"
              onClick={() => {
                setSelectedProcess(entry);
                if (onAlertSelect) {
                  const syntheticAlertId = `${entry.agent_id}-${entry.pid}`;
                  onAlertSelect(syntheticAlertId);
                }
              }}
            >
              <td className="p-2">{entry.agent_id || 'N/A'}</td>
              <td className="p-2">{entry.name || 'N/A'}</td>
              <td className="p-2">{entry.pid}</td>
              <td className="p-2">{entry.command || 'N/A'}</td>
              <td className="p-2">
                <span className={`px-2 py-1 text-xs font-semibold rounded ${getBadgeColor(entry.severity)}`}>
                  {(entry.severity ?? 'low').toUpperCase()}
                </span>
              </td>
              <td className="p-2">
                {entry.timestamp
                  ? new Date(entry.timestamp).toLocaleString()
                  : 'N/A'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {selectedProcess && (
        <ProcessDetailModal
          process={{
            ...selectedProcess,
            memory: selectedProcess.memory ?? 0,
            cpu_usage: selectedProcess.cpu_usage ?? 0,
          }}
          onClose={() => setSelectedProcess(null)}
        />
      )}
    </div>
  );
};

export default ProcessTable;
