// dashboard/src/components/DetectionLogicPanel.tsx
import { useCallback, useEffect, useState } from 'react';
import { apiJson, postJson } from '@/api/client';
import { severityClass } from '@/types/Alert';

interface Condition {
  field: string;
  operator: string;
  value: string | number;
}

interface Rule {
  id: string;
  name: string;
  description: string;
  severity?: string;
  target_os: string[];
  logic: string;
  conditions: Condition[];
}

// The stateless rules the backend checks every stored event against, from
// GET /api/rules.
export default function DetectionLogicPanel() {
  const [rules, setRules] = useState<Rule[]>([]);
  const [status, setStatus] = useState<string | null>(null);

  const load = useCallback(
    () =>
      apiJson<Rule[]>('/api/rules')
        .then(setRules)
        .catch((err) => setStatus(`Could not load rules: ${err.message}`)),
    [],
  );

  useEffect(() => {
    load();
  }, [load]);

  const reload = () =>
    postJson<{ stateless_rules_loaded: number }>('/api/rules/reload', {})
      .then((res) => { setStatus(`Reloaded ${res.stateless_rules_loaded} rules from disk.`); return load(); })
      .catch((err) => setStatus(`Reload refused: ${err.message}. The previous rules stay active.`));

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">Detection Rules</h2>
        <button onClick={reload} className="text-sm px-3 py-1 bg-blue-100 text-blue-800 rounded hover:bg-blue-200">
          Reload from disk
        </button>
      </div>
      {status && <p className="text-sm text-gray-600 mb-4">{status}</p>}
      {rules.length === 0 ? (
        <p className="text-sm text-gray-500">No rules loaded.</p>
      ) : (
        <div className="space-y-4">
          {rules.map((rule) => (
            <div key={rule.id} className="rounded-lg p-4 border">
              <div className="flex justify-between items-center mb-1">
                <h3 className="text-lg font-bold">{rule.id} {rule.name}</h3>
                <span className={`text-xs font-medium uppercase px-2 py-1 rounded ${severityClass(rule.severity)}`}>
                  {rule.severity ?? 'low'}
                </span>
              </div>
              <p className="text-sm text-gray-700">{rule.description}</p>
              <p className="text-xs text-gray-500 mt-2">
                {rule.logic} of:{' '}
                {rule.conditions.map((c) => `${c.field} ${c.operator} ${JSON.stringify(c.value)}`).join(', ')}
                {' '}on {rule.target_os.join(', ')}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
