import React, { useState, useMemo } from 'react';
import AlertTable from './AlertTable';
import AlertModal from './AlertModal';
import EndpointPanel from './EndpointPanel';
import AnomalyChart from './AnomalyChart';
import PluginPanel from './PluginPanel';
// Import existing chart/heatmap components (assuming they exist)
// import AlertsHeatmap from './AlertsHeatmap'; 
// import AlertsBySeverityChart from './AlertsBySeverityChart'; 
// etc.

interface Alert {
  id: number;
  agent: string;
  pid: number;
  processName: string;
  command: string;
  severity: 'Low' | 'Medium' | 'High';
  timestamp: string;
}

// Example initial alerts data (in practice, fetch from API or context)
const initialAlerts: Alert[] = [
  {
    id: 1, agent: 'Agent-001', pid: 1234, processName: 'malware.exe', command: 'C:\\malware.exe -arg', severity: 'High', 
    timestamp: new Date(Date.now() - 3600*1000).toISOString()
  },
  {
    id: 2, agent: 'Agent-002', pid: 2222, processName: 'svchost.exe', command: 'C:\\Windows\\svchost.exe -k netsvcs', severity: 'Medium', 
    timestamp: new Date(Date.now() - 7200*1000).toISOString()
  },
  {
    id: 3, agent: 'Agent-001', pid: 5678, processName: 'rundll32.exe', command: 'rundll32.exe shell32.dll,OpenAs_RunDLL somefile', severity: 'Low', 
    timestamp: new Date(Date.now() - 1800*1000).toISOString()
  },
  {
    id: 4, agent: 'Agent-003', pid: 1234, processName: 'malware.exe', command: '/usr/bin/malware --run', severity: 'High', 
    timestamp: new Date(Date.now() - 300*1000).toISOString()
  },
  // ... more sample alerts
];

const Dashboard: React.FC = () => {
  const [alerts, setAlerts] = useState<Alert[]>(initialAlerts);
  const [selectedAlert, setSelectedAlert] = useState<Alert | null>(null);

  // Compute summary per agent for Endpoint panels
  const agentSummaries = useMemo(() => {
    const summaryMap: { [agentId: string]: {
      count: number, lastSeen: string, severityCounts: {High: number, Medium: number, Low: number}
    } } = {};
    alerts.forEach(alert => {
      const id = alert.agent;
      if (!summaryMap[id]) {
        summaryMap[id] = { count: 0, lastSeen: alert.timestamp, severityCounts: { High: 0, Medium: 0, Low: 0 } };
      }
      // Increment counts
      summaryMap[id].count += 1;
      summaryMap[id].severityCounts[alert.severity] += 1;
      // Update last seen (latest timestamp)
      if (new Date(alert.timestamp) > new Date(summaryMap[id].lastSeen)) {
        summaryMap[id].lastSeen = alert.timestamp;
      }
    });
    // Convert to array of AgentSummary objects
    return Object.entries(summaryMap).map(([agentId, summary], index) => ({
      agentId,
      totalAlerts: summary.count,
      lastSeen: summary.lastSeen,
      severityCounts: summary.severityCounts,
      // Simulate online status: e.g., alternate or based on recent activity
      online: index % 2 === 0  // for demo, every other agent is "online"
    }));
  }, [alerts]);

  const handleRowClick = (alert: Alert) => {
    setSelectedAlert(alert);
  };
  const handleCloseModal = () => {
    setSelectedAlert(null);
  };

  // Stub handlers for action buttons (could integrate with API calls)
  const killProcess = (alert: Alert) => {
    console.log("Kill process action on:", alert);
    // e.g., call API to kill process
  };
  const quarantineEndpoint = (alert: Alert) => {
    console.log("Quarantine action on:", alert);
    // e.g., call API to quarantine endpoint
  };
  const whitelistProcess = (alert: Alert) => {
    console.log("Whitelist action on:", alert);
    // e.g., call API to whitelist the process
  };

  return (
    <div className="p-4">
      {/* Top section: existing charts and new anomaly chart */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
        {/* Preserve existing chart/heatmap components here */}
        {/* Example: <AlertsBySeverityChart data={...} /> */}
        {/* Example: <AlertsHeatmap data={...} /> */}
        <AnomalyChart />
      </div>

      {/* Main content section: Alerts table & side panels */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Alerts Table (occupies two-thirds on large screens) */}
        <div className="lg:col-span-2">
          <AlertTable alerts={alerts} onRowClick={handleRowClick} />
        </div>
        {/* Side column for endpoint panels and plugin panel */}
        <div className="lg:col-span-1 space-y-4">
          {/* Endpoint summary panels */}
          {agentSummaries.map(agent => (
            <EndpointPanel key={agent.agentId} data={agent} />
          ))}
          {/* Plugin module panel */}
          <PluginPanel />
        </div>
      </div>

      {/* Alert Details Modal (overlay) */}
      {selectedAlert && (
        <AlertModal 
          alert={selectedAlert} 
          relatedAlerts={alerts} 
          onClose={handleCloseModal}
          onKill={killProcess}
          onQuarantine={quarantineEndpoint}
          onWhitelist={whitelistProcess}
        />
      )}
    </div>
  );
};

export default Dashboard;


