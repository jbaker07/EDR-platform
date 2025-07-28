import React, { useEffect, useState } from 'react';
import RootCauseTrace from '@/components/RootCauseTrace';
import RootCauseGraph from '@/components/RootCauseGraph';
import TrustHistoryChart from '@/components/TrustHistoryChart';
import GNNReplay from '@/components/GNNReplay';


interface AlertDetail {
  id: number;
  hostname: string;
  process_name: string;
  rule_id: string;
  rule_name: string;
  description: string;
  severity: string;
  timestamp: string;
}

interface Explanation {
  natural_summary: string;
}

interface Props {
  alertId: string;
}

const SOCAssistantPanel: React.FC<Props> = ({ alertId }) => {
  const [alert, setAlert] = useState<AlertDetail | null>(null);
  const [explanation, setExplanation] = useState<Explanation | null>(null);
  const [remediation, setRemediation] = useState<string[]>([]);
  const [actionResult, setActionResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!alertId) return;

    fetch(`/api/alerts/${alertId}`)
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(setAlert)
      .catch(err => setError(err.message));

    fetch(`/api/assistant/explain?event_id=${alertId}`)
      .then(res => {
        if (!res.ok) throw new Error(`Explain API error: ${res.status}`);
        return res.json();
      })
      .then(data => setExplanation({ natural_summary: data.natural_summary }))
      .catch(err => console.error('Explain fetch error:', err));

    fetch(`/api/remediation/suggest?event_id=${alertId}`)
      .then(res => {
        if (!res.ok) throw new Error(`Remediation API error: ${res.status}`);
        return res.json();
      })
      .then(data => setRemediation(data.recommendations || []))
      .catch(() => setRemediation([]));
  }, [alertId]);

  const handleTakeAction = async () => {
    try {
      const res = await fetch(`/api/response/action?alert_id=${alertId}&action=isolate`, {
        method: 'POST',
      });
      const data = await res.json();
      setActionResult(data.justification);
    } catch (err) {
      setActionResult('Failed to execute action.');
    }
  };

  if (error) return <div className="text-red-500 p-4">Error: {error}</div>;
  if (!alert) return <div className="text-gray-500 p-4">Loading alert details...</div>;

  return (
    <div className="bg-white shadow rounded-xl p-6 mt-6">
      <h2 className="text-lg font-bold mb-2">SOC Assistant: Alert #{alert.id}</h2>
      <p className="text-sm text-gray-700 mb-4">{new Date(alert.timestamp).toLocaleString()}</p>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div><strong>Hostname:</strong> {alert.hostname}</div>
        <div><strong>Process:</strong> {alert.process_name}</div>
        <div><strong>Rule Name:</strong> {alert.rule_name}</div>
        <div><strong>Severity:</strong> {alert.severity}</div>
      </div>

      <div className="mb-4">
        <strong>Description:</strong>
        <p className="text-sm text-gray-800">{alert.description}</p>
      </div>

      <div className="mt-6 border-t pt-4">
        <h3 className="font-semibold text-gray-800 mb-1">SOC Explanation</h3>
        {explanation ? (
          <p className="text-gray-700 text-sm">{explanation.natural_summary}</p>
        ) : (
          <p className="text-gray-500 text-sm">Loading explainability insights...</p>
        )}
      </div>

      {remediation.length > 0 && (
        <div className="mt-6 border-t pt-4">
          <h3 className="font-semibold text-gray-800 mb-1">Remediation Recommendations</h3>
          <ul className="list-disc ml-5 text-sm text-gray-700 space-y-1">
            {remediation.map((rec, i) => (
              <li key={i}>{rec}</li>
            ))}
          </ul>
        </div>
      )}

      <RootCauseTrace eventId={alertId} />
      <RootCauseGraph eventId={alertId} />
      <TrustHistoryChart hostname={alert.hostname} />
      <GNNReplay eventId={alertId} />

      <div className="mt-6">
        <button
          onClick={handleTakeAction}
          className="px-4 py-2 rounded bg-red-600 text-white hover:bg-red-700"
        >
          🛡️ Take Action (Isolate)
        </button>

        {actionResult && (
          <p className="mt-2 text-sm text-gray-700 bg-gray-100 rounded p-2">
            {actionResult}
          </p>
        )}
      </div>
    </div>
  );
};

export default SOCAssistantPanel;
