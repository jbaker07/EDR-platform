import React, { useEffect, useState } from 'react';
import Fuse from 'fuse.js';
import axios from 'axios';

interface ProcessEntry {
  agent_id: string;
  pid: number;
  name: string;
  command: string;
  severity: string;
  timestamp: string;
}

const getBadgeColor = (severity: string) => {
  switch (severity.toLowerCase()) {
    case 'high':
      return 'bg-red-100 text-red-800';
    case 'medium':
      return 'bg-yellow-100 text-yellow-800';
    case 'low':
    default:
      return 'bg-green-100 text-green-800';
  }
};

const ProcessTable: React.FC = () => {
  const [data, setData] = useState<ProcessEntry[]>([]);
  const [query, setQuery] = useState('');
  const [filtered, setFiltered] = useState<ProcessEntry[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      const res = await axios.get<ProcessEntry[]>('http://localhost:8000/telemetry/all');
      setData(res.data);
      setFiltered(res.data);
    };
    fetchData();
  }, []);

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
      <div className="mb-4">
        <input
          type="text"
          placeholder="Search processes..."
          className="w-full border p-2 rounded"
          value={query}
          onChange={e => setQuery(e.target.value)}
        />
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
            <tr key={i} className="border-b hover:bg-gray-50">
              <td className="p-2">{entry.agent_id}</td>
              <td className="p-2">{entry.name}</td>
              <td className="p-2">{entry.pid}</td>
              <td className="p-2">{entry.command}</td>
              <td className="p-2">
                <span className={`px-2 py-1 text-xs font-semibold rounded ${getBadgeColor(entry.severity)}`}>
                  {entry.severity}
                </span>
              </td>
              <td className="p-2">{new Date(entry.timestamp).toLocaleString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default ProcessTable;
