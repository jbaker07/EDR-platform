// src/components/RiskSummaryPanel.tsx
import { SEVERITIES, StoredAlert, severityClass } from '@/types/Alert';

interface RiskSummaryPanelProps {
  alerts: StoredAlert[];
}

// Counts of stored alerts by the severity their rule assigned.
export default function RiskSummaryPanel({ alerts }: RiskSummaryPanelProps) {
  const counts: Record<string, number> = {};
  for (const a of alerts) {
    const sev = a.severity.toLowerCase();
    counts[sev] = (counts[sev] ?? 0) + 1;
  }

  return (
    <div>
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {SEVERITIES.map((sev) => (
          <div key={sev} className={`${severityClass(sev)} p-4 rounded shadow text-center`}>
            <p className="text-sm capitalize">{sev} alerts</p>
            <p className="text-2xl font-bold">{counts[sev] ?? 0}</p>
          </div>
        ))}
      </div>
      <p className="text-xs text-gray-500 mt-2">From the newest {alerts.length} stored alerts.</p>
    </div>
  );
}
