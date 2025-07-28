import React from 'react';

interface AgentSummary {
  agentId: string;
  totalAlerts: number;
  lastSeen: string;
  severityCounts: {
    high: number;
    medium: number;
    low: number;
  };
  online: boolean;
}

interface EndpointPanelProps {
  data: AgentSummary;
}

const EndpointPanel: React.FC<EndpointPanelProps> = ({ data }) => {
  const { agentId, totalAlerts, lastSeen, severityCounts, online } = data;
  return (
    <div className="bg-white rounded shadow p-4 text-sm">
      <div className="flex items-center justify-between mb-2">
        <h3 className="font-semibold text-base">{agentId}</h3>
        <span
          className={`inline-block w-3 h-3 rounded-full ${online ? 'bg-green-500' : 'bg-red-500'}`}
          title={online ? 'Online' : 'Offline'}
        />
      </div>
      <p><strong>Total Alerts:</strong> {totalAlerts}</p>
      <p><strong>Last Alert:</strong> { lastSeen ? new Date(Number(lastSeen) * 1000).toLocaleString() : 'N/A' }</p>
      <div className="mt-2">
        <p><strong>Severity Breakdown:</strong></p>
        <ul className="ml-4 list-disc">
          <li>High: {severityCounts.high}</li>
          <li>Medium: {severityCounts.medium}</li>
          <li>Low: {severityCounts.low}</li>
        </ul>
      </div>
    </div>
  );
};

export default EndpointPanel;
