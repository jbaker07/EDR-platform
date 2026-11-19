// src/pages/Dashboard.tsx
import React, { useEffect, useState } from 'react';
import AlertTable from '@/components/AlertTable';
import TelemetryTimeline from '@/components/TelemetryTimeline';
import RiskSummaryPanel from '@/components/RiskSummaryPanel';
import AgentRegistryPanel from '@/components/AgentRegistryPanel';
import TrustPanel from '@/components/TrustPanel';
import ExplainPanel from '@/components/ExplainPanel';
import GlobalRiskHeatmap from '@/components/GlobalRiskHeatmap';
import ArchivedReplayViewer from '@/components/ArchivedReplayViewer';

import { Alert } from '@/types/Alert';

export default function DashboardPage() {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [selectedAlert, setSelectedAlert] = useState<any | null>(null);
  // Host of the newest telemetry row. "dashboard-collector" is only the label
  // /api/telemetry/all puts on its response, not a host with a trust score.
  const [trustHost, setTrustHost] = useState<string | null>(null);

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
      const newest = data.processes[data.processes.length - 1];
      if (newest?.hostname) setTrustHost(newest.hostname);
    };

    fetchLive();
    const id = setInterval(fetchLive, 5000);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="p-6 space-y-6">
      <RiskSummaryPanel alerts={alerts} />
      {trustHost && <TrustPanel hostname={trustHost} />}
      <GlobalRiskHeatmap />
      <TelemetryTimeline alerts={alerts} />
      <ArchivedReplayViewer />
      
      <h2 className="text-xl font-semibold">Live Alerts</h2>
      <AlertTable alerts={alerts} onRowClick={setSelectedAlert} />

      {/* Inline ExplainPanel for selected alert */}
      <ExplainPanel
        selectedAlert={selectedAlert}
        features={[selectedAlert?.cpu_usage || 0, selectedAlert?.memory || 0]}
        featureNames={['cpu_usage', 'memory']}
      />

      <AgentRegistryPanel />
    </div>
  );
}
