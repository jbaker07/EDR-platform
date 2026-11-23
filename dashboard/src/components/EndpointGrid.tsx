// src/components/EndpointGrid.tsx
import { useEffect, useState } from 'react';
import { apiJson } from '@/api/client';
import { HostSummary, severityClass } from '@/types/Alert';

// One card per host from /api/telemetry/agents. Every number is a count of
// stored rows in the last 15 minutes.
export default function EndpointGrid() {
  const [hosts, setHosts] = useState<HostSummary[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const load = () =>
      apiJson<HostSummary[]>('/api/telemetry/agents')
        .then((data) => { setHosts(data); setError(null); })
        .catch((err) => setError(err.message));

    load();
    const id = setInterval(load, 10000);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="bg-white p-6 rounded-xl shadow">
      <h2 className="text-xl font-semibold mb-4">Endpoints (last 15 minutes)</h2>
      {error && <p className="text-sm text-red-600">Could not load endpoints: {error}</p>}
      {hosts.length === 0 ? (
        <p className="text-gray-500 text-sm">No endpoint has sent telemetry in the last 15 minutes.</p>
      ) : (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2 xl:grid-cols-3">
          {hosts.map((h) => (
            <div key={h.hostname} className="bg-gray-50 p-4 rounded-lg border">
              <div className="flex justify-between items-center mb-2">
                <h3 className="font-bold">{h.hostname}</h3>
                <span
                  className={`px-2 py-1 text-xs font-semibold rounded text-white ${
                    h.status === 'online' ? 'bg-green-500' : 'bg-gray-400'
                  }`}
                >
                  {h.status.toUpperCase()}
                </span>
              </div>
              <p className="text-sm text-gray-700">
                Last seen: {h.last_seen ? new Date(h.last_seen * 1000).toLocaleString() : 'no telemetry in window'}
              </p>
              <p className="text-sm text-gray-700">Process events: {h.process_events}</p>
              <p className="text-sm mt-2">
                Alerts: {h.alert_count}{' '}
                {h.worst_severity && (
                  <span className={`ml-1 px-2 py-0.5 rounded text-xs font-semibold ${severityClass(h.worst_severity)}`}>
                    worst {h.worst_severity}
                  </span>
                )}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
