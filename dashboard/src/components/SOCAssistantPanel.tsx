import React, { useEffect, useState } from 'react';

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

interface Props {
  alertId: string;
}

const SOCAssistantPanel: React.FC<Props> = ({ alertId }) => {
  const [alert, setAlert] = useState<AlertDetail | null>(null);
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
  }, [alertId]);

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
        <h3 className="font-semibold text-gray-800 mb-1">SOC Explanation (Coming Soon):</h3>
        <p className="text-gray-600 text-sm">
          This section will provide AI-assisted explanations for what triggered the alert,
          including system behavior, detection logic, and next steps for remediation or triage.
        </p>
      </div>
    </div>
  );
};

export default SOCAssistantPanel;

