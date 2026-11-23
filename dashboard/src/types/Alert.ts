// src/types/Alert.ts
// Shapes of what the backend stores and serves. No field here is computed in
// the browser.

// One row of /api/alerts: a stored rule match.
export interface StoredAlert {
  id: number;
  hostname: string;
  process_name: string;
  rule_id: string;
  rule_name: string;
  description: string;
  severity: string;
  timestamp: number; // epoch seconds
  created_at: number;
}

// One row of /api/telemetry/all: a stored process event.
export interface ProcessEvent {
  hostname: string;
  endpoint_id: string;
  os_type: string;
  pid: number;
  process_name: string;
  cmd: string;
  cpu_percent: number;
  memory: number; // bytes
  timestamp: number; // epoch seconds
}

// One row of /api/telemetry/agents: counts of stored rows for one host.
export interface HostSummary {
  hostname: string;
  last_seen: number | null;
  process_events: number;
  alert_count: number;
  alerts_by_severity: Record<string, number>;
  worst_severity: string | null;
  status: 'online' | 'offline';
}

export const SEVERITIES = ['critical', 'high', 'medium', 'low'] as const;

export function severityClass(severity: string | null | undefined): string {
  switch ((severity ?? '').toLowerCase()) {
    case 'critical':
      return 'bg-red-600 text-white';
    case 'high':
      return 'bg-red-100 text-red-800';
    case 'medium':
      return 'bg-yellow-100 text-yellow-800';
    case 'low':
      return 'bg-green-100 text-green-800';
    default:
      return 'bg-gray-100 text-gray-600';
  }
}
