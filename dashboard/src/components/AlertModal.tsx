import React from 'react';
import { XMarkIcon } from '@heroicons/react/24/outline';

interface Alert {
  id: number;
  agent: string;
  pid: number;
  processName: string;
  command: string;
  severity: 'Low' | 'Medium' | 'High';
  timestamp: string;
}

interface AlertModalProps {
  alert: Alert;
  relatedAlerts: Alert[];         // list of alerts (including alert) to find correlations
  onClose: () => void;
  onKill?: (alert: Alert) => void;
  onQuarantine?: (alert: Alert) => void;
  onWhitelist?: (alert: Alert) => void;
}

const AlertModal: React.FC<AlertModalProps> = ({
  alert,
  relatedAlerts,
  onClose,
  onKill,
  onQuarantine,
  onWhitelist
}) => {
  if (!alert) return null;

  // Derive related alerts (same process name or same PID, different agent or process)
  const correlated = relatedAlerts.filter(a =>
    (a.processName === alert.processName || a.pid === alert.pid) && a.id !== alert.id
  );

  // Helper to get severity badge style
  const severityBadge = (sev: string) => {
    const base = "px-2 py-0.5 text-xs font-semibold rounded";
    if (sev === 'High') return base + " bg-red-100 text-red-800";
    if (sev === 'Medium') return base + " bg-yellow-100 text-yellow-800";
    if (sev === 'Low') return base + " bg-green-100 text-green-800";
    return base;
  };

  return (
    <div 
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" 
      onClick={onClose}
    >
      {/* Modal Card */}
      <div 
        className="bg-white rounded shadow-lg max-w-lg w-full p-6 relative" 
        onClick={e => e.stopPropagation()}  // prevent backdrop click from closing when clicking inside
      >
        {/* Close Button */}
        <button onClick={onClose} className="absolute top-2 right-2 text-gray-500 hover:text-gray-800">
          {/* Using a text "X" if no icon library: */}
          × 
        </button>

        {/* Modal Header */}
        <div className="mb-4">
          <h2 className="text-xl font-semibold mb-2 flex items-center">
            Alert Details 
            <span className={`${severityBadge(alert.severity)} ml-3`}>
              {alert.severity}
            </span>
          </h2>
          <p className="text-gray-600">{alert.processName}</p>
        </div>

        {/* Alert Information */}
        <div className="text-sm space-y-1 mb-4">
          <p><strong>Agent ID:</strong> {alert.agent}</p>
          <p><strong>Process ID:</strong> {alert.pid}</p>
          <p><strong>Command Line:</strong> {alert.command}</p>
          <p><strong>Timestamp:</strong> {new Date(alert.timestamp).toLocaleString()}</p>
        </div>

        {/* Action Buttons */}
        <div className="flex space-x-3 mb-4">
          <button 
            className="px-3 py-1 bg-red-600 text-white text-sm rounded hover:bg-red-700"
            onClick={() => onKill ? onKill(alert) : alertStub('Kill Process', alert)}
          >
            Kill Process
          </button>
          <button 
            className="px-3 py-1 bg-yellow-500 text-white text-sm rounded hover:bg-yellow-600"
            onClick={() => onQuarantine ? onQuarantine(alert) : alertStub('Quarantine', alert)}
          >
            Quarantine
          </button>
          <button 
            className="px-3 py-1 bg-gray-500 text-white text-sm rounded hover:bg-gray-600"
            onClick={() => onWhitelist ? onWhitelist(alert) : alertStub('Whitelist', alert)}
          >
            Whitelist
          </button>
        </div>

        {/* Correlated Alerts Section */}
        <div className="mt-4">
          <h3 className="font-medium mb-2">Related Alerts:</h3>
          {correlated.length > 0 ? (
            <ul className="list-disc list-inside text-sm text-gray-700 max-h-32 overflow-y-auto">
              {correlated.map(rel => (
                <li key={rel.id}>
                  Agent <strong>{rel.agent}</strong>, PID {rel.pid} – 
                  <span className={`ml-1 ${severityBadge(rel.severity)}`}>{rel.severity}</span>
                  <span className="ml-1">({new Date(rel.timestamp).toLocaleDateString()})</span>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-gray-500">No other alerts with this process.</p>
          )}
        </div>

        {/* (Optional) Network Graph Placeholder */}
        {correlated.length > 0 && (
          <div className="mt-4 p-3 bg-gray-100 rounded">
            <p className="text-sm text-gray-600 italic text-center">[Network graph view placeholder]</p>
          </div>
        )}
      </div>
    </div>
  );
};

// Fallback stub action handler
function alertStub(action: string, alert: Alert) {
  console.log(`${action} clicked for Alert ${alert.id} (agent ${alert.agent})`);
  window.alert(`${action} action triggered (stub) for agent ${alert.agent}`);
}

export default AlertModal;
