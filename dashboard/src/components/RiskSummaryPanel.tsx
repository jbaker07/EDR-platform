// src/components/RiskSummaryPanel.tsx
import React from 'react';
import { Alert } from '@/types/Alert';

interface RiskSummaryPanelProps {
  alerts: Alert[];
}

export default function RiskSummaryPanel({ alerts }: RiskSummaryPanelProps) {
  const counts = alerts
    .flatMap(a => a.processes)
    .reduce(
      (acc, p) => {
        switch (p.risk_level) {
          case 'high':
            acc.high++;
            break;
          case 'medium':
            acc.medium++;
            break;
          default:
            acc.low++;
        }
        return acc;
      },
      { low: 0, medium: 0, high: 0 }
    );

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div className="bg-green-100 text-green-800 p-4 rounded shadow text-center">
        <p className="text-sm">🟢 Low Risk</p>
        <p className="text-2xl font-bold">{counts.low}</p>
      </div>
      <div className="bg-yellow-100 text-yellow-800 p-4 rounded shadow text-center">
        <p className="text-sm">🟡 Medium Risk</p>
        <p className="text-2xl font-bold">{counts.medium}</p>
      </div>
      <div className="bg-red-100 text-red-800 p-4 rounded shadow text-center">
        <p className="text-sm">🔴 High Risk</p>
        <p className="text-2xl font-bold">{counts.high}</p>
      </div>
    </div>
  );
}
