import { useCallback, useEffect, useMemo, useState } from 'react';
import Fuse from 'fuse.js';
import { apiJson } from '@/api/client';
import { ProcessEvent } from '@/types/Alert';

interface TelemetryResponse {
  timestamp: number;
  processes: ProcessEvent[];
}

// Stored process events from /api/telemetry/all (last 15 minutes), as stored.
export default function ProcessTable() {
  const [data, setData] = useState<ProcessEvent[]>([]);
  const [query, setQuery] = useState('');
  const [live, setLive] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(() => {
    apiJson<TelemetryResponse>('/api/telemetry/all')
      .then((res) => { setData(res.processes); setError(null); })
      .catch((err) => setError(err.message));
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  useEffect(() => {
    if (!live) return;
    const interval = setInterval(fetchData, 10000);
    return () => clearInterval(interval);
  }, [live, fetchData]);

  const filtered = useMemo(() => {
    if (!query.trim()) return data;
    const fuse = new Fuse(data, { keys: ['process_name', 'cmd', 'hostname'], threshold: 0.3 });
    return fuse.search(query).map((r) => r.item);
  }, [query, data]);

  return (
    <div className="bg-white shadow rounded-xl p-6 mb-10">
      <div className="flex justify-between items-center mb-4">
        <input
          type="text"
          placeholder="Search processes..."
          className="w-full max-w-sm border p-2 rounded"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        <label className="flex items-center space-x-2 text-sm ml-4">
          <input
            type="checkbox"
            checked={live}
            onChange={() => setLive((prev) => !prev)}
            className="accent-blue-600"
          />
          <span>Live Updates</span>
        </label>
      </div>

      {error && <p className="text-sm text-red-600 mb-2">Could not load telemetry: {error}</p>}
      <p className="text-xs text-gray-500 mb-2">{filtered.length} of {data.length} stored process events.</p>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="text-left border-b">
              <th className="p-2">Host</th>
              <th className="p-2">Process</th>
              <th className="p-2">PID</th>
              <th className="p-2">Command</th>
              <th className="p-2">CPU %</th>
              <th className="p-2">Memory</th>
              <th className="p-2">Time</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((p, i) => (
              <tr key={`${p.hostname}-${p.pid}-${p.timestamp}-${i}`} className="border-b hover:bg-gray-50">
                <td className="p-2">{p.hostname}</td>
                <td className="p-2">{p.process_name}</td>
                <td className="p-2">{p.pid}</td>
                <td className="p-2 font-mono text-xs break-all">{p.cmd}</td>
                <td className="p-2">{p.cpu_percent.toFixed(1)}</td>
                <td className="p-2">{(p.memory / 1024 / 1024).toFixed(1)} MB</td>
                <td className="p-2">{new Date(p.timestamp * 1000).toLocaleString()}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
