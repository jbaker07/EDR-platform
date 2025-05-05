// src/pages/Dashboard.tsx
import React, { useEffect, useState } from 'react';
import AlertTable from '@/components/AlertTable';
import TelemetryTimeline from '@/components/TelemetryTimeline';
import RiskSummaryPanel from '@/components/RiskSummaryPanel';
import AgentRegistryPanel from '@/components/AgentRegistryPanel';
import TrustPanel from '@/components/TrustPanel';
import ExplainablePanel from '@/components/ExplainablePanel';  // ✅ NEW

import { Alert } from '@/types/Alert';

export default function DashboardPage() {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [selectedAlert, setSelectedAlert] = useState<any | null>(null);  // ✅ Alert selected for explanation

  useEffect(() => {
    const fetchLive = async () => {
      const res = await fetch('/api/telemetry/all');
      const data = await res.json();
      const procs = data.processes.map((p: any) => ({
        pid: p.pid,
        name: p.process_name,
        cmd: p.cmd,
        cpu_usage: p.cpu_percent,
        memory: p.memory,
        risk_score: Math.round(p.cpu_percent),
        risk_level:
          p.cpu_percent > 80 ? 'high' :
          p.cpu_percent > 50 ? 'medium' :
          'low',
        history: p.history || [],
        timestamp: data.timestamp,
        hostname: data.hostname
      }));
      setAlerts([{ hostname: data.hostname, timestamp: data.timestamp, processes: procs }]);
    };

    fetchLive();
    const id = setInterval(fetchLive, 5000);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="p-6 space-y-6">
      <RiskSummaryPanel alerts={alerts} />
      <TrustPanel hostname="dashboard-collector" />
      
      <TelemetryTimeline alerts={alerts} />

      <h2 className="text-xl font-semibold">Live Alerts</h2>
      <AlertTable alerts={alerts} onRowClick={setSelectedAlert} />  {/* ✅ Support selecting alert */}

      <ExplainablePanel selectedAlert={selectedAlert} /> {/* ✅ Shows explanation for selected alert */}

      <AgentRegistryPanel />
    </div>
  );
}
