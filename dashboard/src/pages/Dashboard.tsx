// src/pages/Dashboard.tsx
import { useEffect, useState } from 'react';
import AlertTable from '@/components/AlertTable';
import RiskSummaryPanel from '@/components/RiskSummaryPanel';
import EndpointGrid from '@/components/EndpointGrid';
import AgentRegistryPanel from '@/components/AgentRegistryPanel';
import { apiJson } from '@/api/client';
import { StoredAlert } from '@/types/Alert';

export default function DashboardPage() {
  const [alerts, setAlerts] = useState<StoredAlert[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const load = () =>
      apiJson<StoredAlert[]>('/api/alerts')
        .then((data) => { setAlerts(data); setError(null); })
        .catch((err) => setError(err.message));

    load();
    const id = setInterval(load, 5000);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="p-6 space-y-6">
      {error && <p className="text-sm text-red-600">Could not load alerts: {error}</p>}
      <RiskSummaryPanel alerts={alerts} />
      <EndpointGrid />
      <AlertTable alerts={alerts} />
      <AgentRegistryPanel />
    </div>
  );
}
