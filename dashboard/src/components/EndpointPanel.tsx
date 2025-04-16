import React from 'react';

interface AgentSummary {
  agentId: string;
  totalAlerts: number;
  lastSeen: string;  // timestamp of last alert or last activity
  severityCounts: {
    High: number;
    Medium: number;
    Low: number;
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
        {/* Status dot */}
        <span 
          className={`inline-block w-3 h-3 rounded-full ${online ? 'bg-green-500' : 'bg-red-500'}`} 
          title={online ? 'Online' : 'Offline'}
        />
      </div>
      <p><strong>Total Alerts:</strong> {totalAlerts}</p>
      <p><strong>Last Alert:</strong> { lastSeen ? new Date(lastSeen).toLocaleString() : 'N/A' }</p>
      <div className="mt-2">
        <p><strong>Severity Breakdown:</strong></p>
        <ul className="ml-4 list-disc">
          <li>High: {severityCounts.High}</li>
          <li>Medium: {severityCounts.Medium}</li>
          <li>Low: {severityCounts.Low}</li>
        </ul>
      </div>
    </div>
  );
};

export default EndpointPanel;
