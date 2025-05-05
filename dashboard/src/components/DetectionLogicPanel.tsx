// dashboard/src/components/DetectionLogicPanel.tsx
import React from 'react';
import { ShieldCheck, Brain, AlertCircle } from 'lucide-react';

const detectionRules = [
  {
    id: 'rule-001',
    name: 'High CPU Usage',
    description: 'Detects processes with CPU usage > 90%',
    impact: 'high',
  },
  {
    id: 'rule-002',
    name: 'Unauthorized USB Insert',
    description: 'Flags unknown USB devices not on the allowlist',
    impact: 'medium',
  },
  {
    id: 'rule-003',
    name: 'Fileless Execution',
    description: 'Detects script-based memory-only execution patterns',
    impact: 'high',
  },
  {
    id: 'rule-004',
    name: 'Network Beaconing',
    description: 'Analyzes periodic outbound connections to rare hosts',
    impact: 'medium',
  },
];

const getImpactColor = (impact: string) => {
  return impact === 'high'
    ? 'bg-red-100 text-red-800 border-l-4 border-red-600'
    : 'bg-yellow-100 text-yellow-800 border-l-4 border-yellow-500';
};

export default function DetectionLogicPanel() {
  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <div className="flex items-center gap-2 mb-4">
        <Brain className="text-indigo-600" />
        <h2 className="text-xl font-semibold">🧠 Detection Logic Rules</h2>
      </div>
      <div className="space-y-4">
        {detectionRules.map(rule => (
          <div
            key={rule.id}
            className={`rounded-lg p-4 ${getImpactColor(rule.impact)} hover:shadow transition`}
          >
            <div className="flex justify-between items-center mb-1">
              <h3 className="text-lg font-bold">{rule.name}</h3>
              <span className="text-xs font-medium uppercase">{rule.impact}</span>
            </div>
            <p className="text-sm text-gray-700">{rule.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
