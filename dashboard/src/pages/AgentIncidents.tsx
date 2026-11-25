// src/pages/AgentIncidents.tsx
import { useEffect, useState } from 'react';
import { apiJson } from '@/api/client';
import { severityClass } from '@/types/Alert';

// One incident the agent forwarded (backend/app/routes/incident_router.py).
interface AgentIncident {
  key: string;
  id: string;
  host: string;
  title: string;
  sources: string[];
  exe: string;
  primary_technique: string;
  techniques: string[];
  severity: string;
  risk: number;
  alerts_count: number;
  first_ts: number;
  last_ts: number;
  details: Record<string, unknown>;
}

// The detail line for a row: the guard's caller, target and mode, or the playbook.
function detailText(d: Record<string, unknown>): string {
  if (typeof d.guard === 'string') {
    return `${d.guard} ${String(d.mode ?? '')}: ${String(d.caller_exe ?? '')} on ${String(d.target ?? '')}`;
  }
  if (typeof d.playbook_id === 'string') {
    return d.playbook_id;
  }
  return '';
}

// Incidents raised on the endpoints by the agent's rules, playbooks and guard,
// newest first. Refreshed every 10 s.
export default function AgentIncidents() {
  const [items, setItems] = useState<AgentIncident[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let live = true;
    const load = () =>
      apiJson<AgentIncident[]>('/api/incidents')
        .then((rows) => {
          if (live) {
            setItems(rows);
            setError(null);
          }
        })
        .catch((e: Error) => live && setError(e.message));
    load();
    const t = setInterval(load, 10000);
    return () => {
      live = false;
      clearInterval(t);
    };
  }, []);

  return (
    <div className="bg-white p-6 rounded-xl shadow">
      <h2 className="text-xl font-semibold mb-4">Agent incidents</h2>
      {error && <p className="text-sm text-red-600 mb-2">Could not load incidents: {error}</p>}
      {items.length === 0 ? (
        <p className="text-sm text-gray-500">No incidents from agents yet.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm text-left">
            <thead className="bg-gray-100 text-xs uppercase">
              <tr>
                <th className="px-4 py-2">Last seen</th>
                <th className="px-4 py-2">Host</th>
                <th className="px-4 py-2">Incident</th>
                <th className="px-4 py-2">Source</th>
                <th className="px-4 py-2">Technique</th>
                <th className="px-4 py-2">Alerts</th>
                <th className="px-4 py-2">Severity</th>
              </tr>
            </thead>
            <tbody>
              {items.map((i) => (
                <tr key={i.key} className="border-t align-top">
                  <td className="px-4 py-2 whitespace-nowrap">{new Date(i.last_ts * 1000).toLocaleString()}</td>
                  <td className="px-4 py-2">{i.host}</td>
                  <td className="px-4 py-2">
                    <div className="font-medium">{i.title}</div>
                    <div className="text-xs text-gray-500">{i.exe}</div>
                    <div className="text-xs text-gray-500">{detailText(i.details)}</div>
                  </td>
                  <td className="px-4 py-2">{i.sources.join(', ')}</td>
                  <td className="px-4 py-2">{i.techniques.join(', ')}</td>
                  <td className="px-4 py-2">{i.alerts_count}</td>
                  <td className="px-4 py-2">
                    <span className={`px-2 py-1 rounded text-xs font-semibold ${severityClass(i.severity)}`}>
                      {i.severity.toUpperCase()}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
