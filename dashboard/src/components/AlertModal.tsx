import React from 'react';
import type { Alert } from '@/types/Alert'; // ✅ absolute path using alias if configured
 // ✅ adjust path to wherever your Alert interface is defined

interface AlertModalProps {
  alert: Alert;
  relatedAlerts: Alert[];
  onClose: () => void;
  onKill: (alert: Alert) => void;
  onQuarantine: (alert: Alert) => void;
  onWhitelist: (alert: Alert) => void;
}

const AlertModal: React.FC<AlertModalProps> = ({
  alert,
  relatedAlerts,
  onClose,
  onKill,
  onQuarantine,
  onWhitelist
}) => {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white rounded-lg shadow-lg max-w-2xl w-full p-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold">🚨 Alert Details</h2>
          <button onClick={onClose} className="text-red-500 font-bold text-lg">×</button>
        </div>

        <div className="mb-4">
          <p><strong>Hostname:</strong> {alert.hostname}</p>
          <p><strong>Timestamp:</strong> {new Date(alert.timestamp * 1000).toLocaleString()}</p>
          <p><strong>Total Processes:</strong> {alert.processes.length}</p>
        </div>

        <div className="overflow-y-auto max-h-64 mb-4">
          <table className="min-w-full text-sm">
            <thead>
              <tr className="bg-gray-200">
                <th className="px-2 py-1">PID</th>
                <th className="px-2 py-1">Process Name</th>
                <th className="px-2 py-1">Command</th>
                <th className="px-2 py-1">Severity</th>
              </tr>
            </thead>
            <tbody>
              {alert.processes.map((proc, idx) => (
                <tr key={idx} className="border-b">
                  <td className="px-2 py-1">{proc.pid}</td>
                  <td className="px-2 py-1">{proc.name}</td>
                  <td className="px-2 py-1 truncate max-w-xs">{proc.cmd}</td>
                  <td className="px-2 py-1">
                    <span className={`px-2 py-1 rounded text-xs font-semibold ${
                      proc.risk_level === 'high'
                        ? 'bg-red-500 text-white'
                        : proc.risk_level === 'medium'
                        ? 'bg-yellow-300 text-black'
                        : 'bg-green-400 text-black'
                    }`}>
                      {proc.risk_level?.toUpperCase() || 'UNKNOWN'}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="flex justify-end gap-3">
          <button onClick={() => onKill(alert)} className="bg-red-600 text-white px-3 py-1 rounded hover:bg-red-700">
            🛑 Kill
          </button>
          <button onClick={() => onQuarantine(alert)} className="bg-yellow-500 text-black px-3 py-1 rounded hover:bg-yellow-600">
            🚫 Quarantine
          </button>
          <button onClick={() => onWhitelist(alert)} className="bg-green-500 text-white px-3 py-1 rounded hover:bg-green-600">
            ✅ Whitelist
          </button>
        </div>
      </div>
    </div>
  );
};

export default AlertModal;
