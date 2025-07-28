import React from 'react';
import { X } from 'lucide-react';
import { motion } from 'framer-motion';

interface Endpoint {
  id: string;
  hostname: string;
  ip: string;
  status: 'online' | 'offline';
  riskLevel: 'low' | 'medium' | 'high';
}

interface Props {
  endpoint: Endpoint;
  onClose: () => void;
}

const getRiskBadge = (level: string) => {
  return level === 'high'
    ? 'bg-red-100 text-red-800'
    : level === 'medium'
    ? 'bg-yellow-100 text-yellow-800'
    : 'bg-green-100 text-green-800';
};

const mockProcesses = [
  { pid: 101, name: 'chrome.exe', cpu: 12.3, memory: 512000, severity: 'low' },
  { pid: 202, name: 'malicious.exe', cpu: 89.5, memory: 1228800, severity: 'high' },
];

export default function EndpointModal({ endpoint, onClose }: Props) {
  return (
    <div className="fixed inset-0 z-50 bg-black bg-opacity-30 flex justify-center items-center">
      <motion.div
        className="w-full max-w-3xl bg-white shadow-xl rounded-xl p-6"
        initial={{ opacity: 0, y: 30 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: 30 }}
      >
        {/* Header */}
        <div className="flex justify-between items-center border-b pb-3">
          <div>
            <h2 className="text-xl font-bold">{endpoint.hostname}</h2>
            <p className="text-sm text-gray-600">IP: {endpoint.ip}</p>
          </div>
          <div className="flex items-center gap-2">
            <span className={`text-xs px-2 py-1 rounded ${getRiskBadge(endpoint.riskLevel)}`}>
              {endpoint.riskLevel.toUpperCase()}
            </span>
            <button onClick={onClose}>
              <X className="w-5 h-5 text-gray-500 hover:text-red-600" />
            </button>
          </div>
        </div>

        {/* Risk Trend Placeholder */}
        <div className="mt-4">
          <p className="text-sm font-semibold mb-2">📈 Risk Score Trend (Placeholder)</p>
          <div className="bg-gray-100 p-4 rounded h-24 text-center text-gray-500">[Chart Coming Soon]</div>
        </div>

        {/* Process Table */}
        <div className="mt-6">
          <p className="text-sm font-semibold mb-2">🧠 Recent Processes</p>
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead>
                <tr className="text-gray-600 border-b">
                  <th className="p-2">PID</th>
                  <th className="p-2">Name</th>
                  <th className="p-2">CPU %</th>
                  <th className="p-2">Memory</th>
                  <th className="p-2">Severity</th>
                </tr>
              </thead>
              <tbody>
                {mockProcesses.map(proc => (
                  <tr key={proc.pid} className="border-b hover:bg-gray-50">
                    <td className="p-2">{proc.pid}</td>
                    <td className="p-2">{proc.name}</td>
                    <td className="p-2">{proc.cpu}%</td>
                    <td className="p-2">{(proc.memory / 1024 / 1024).toFixed(2)} MB</td>
                    <td className="p-2 capitalize">{proc.severity}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Actions */}
        <div className="flex justify-end gap-3 mt-6 border-t pt-4">
          <button className="bg-blue-100 text-blue-800 px-4 py-2 rounded hover:bg-blue-200 text-sm">
            🛡️ Quarantine Host
          </button>
          <button className="bg-yellow-100 text-yellow-800 px-4 py-2 rounded hover:bg-yellow-200 text-sm">
            📋 Run Playbook
          </button>
        </div>
      </motion.div>
    </div>
  );
}
