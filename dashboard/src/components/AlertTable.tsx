// src/components/AlertTable.tsx
import { useState } from 'react';
import { SEVERITIES, StoredAlert, severityClass } from '@/types/Alert';

interface AlertTableProps {
  alerts: StoredAlert[];
}

// Stored rule matches from /api/alerts, newest first.
export default function AlertTable({ alerts }: AlertTableProps) {
  const [severity, setSeverity] = useState<string>('all');
  const [search, setSearch] = useState('');

  const q = search.toLowerCase();
  const shown = alerts.filter(
    (a) =>
      (severity === 'all' || a.severity.toLowerCase() === severity) &&
      (a.hostname.toLowerCase().includes(q) ||
        a.process_name.toLowerCase().includes(q) ||
        a.rule_name.toLowerCase().includes(q)),
  );

  return (
    <div className="bg-white p-6 rounded-xl shadow">
      <h2 className="text-xl font-semibold mb-4">Alerts</h2>
      <div className="flex flex-wrap gap-4 items-center mb-4 text-sm">
        <label>
          Severity:
          <select className="ml-2 p-1 border rounded" value={severity} onChange={(e) => setSeverity(e.target.value)}>
            <option value="all">All</option>
            {SEVERITIES.map((s) => (
              <option key={s} value={s}>{s}</option>
            ))}
          </select>
        </label>
        <input
          className="p-1 border border-gray-300 rounded"
          placeholder="Search host, process or rule"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      {shown.length === 0 ? (
        <p className="text-sm text-gray-500">No stored alerts match.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm text-left">
            <thead className="bg-gray-100 text-xs uppercase">
              <tr>
                <th className="px-4 py-2">Time</th>
                <th className="px-4 py-2">Host</th>
                <th className="px-4 py-2">Process</th>
                <th className="px-4 py-2">Rule</th>
                <th className="px-4 py-2">Severity</th>
              </tr>
            </thead>
            <tbody>
              {shown.map((a) => (
                <tr key={a.id} className="border-t" title={a.description}>
                  <td className="px-4 py-2">{new Date(a.timestamp * 1000).toLocaleString()}</td>
                  <td className="px-4 py-2">{a.hostname}</td>
                  <td className="px-4 py-2">{a.process_name}</td>
                  <td className="px-4 py-2">{a.rule_id} {a.rule_name}</td>
                  <td className="px-4 py-2">
                    <span className={`px-2 py-1 rounded text-xs font-semibold ${severityClass(a.severity)}`}>
                      {a.severity.toUpperCase()}
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
