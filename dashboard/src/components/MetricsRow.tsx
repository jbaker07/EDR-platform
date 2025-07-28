// src/components/MetricsRow.tsx
import React, { useEffect, useState } from 'react';
import MetricCard from './MetricCard';
import { Activity, ShieldAlert, Server } from 'lucide-react';
import axios from 'axios';

export default function MetricsRow() {
  const [alerts, setAlerts] = useState(0);
  const [endpoints, setEndpoints] = useState(0);
  const [highRisk, setHighRisk] = useState(0);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const res = await axios.get('http://localhost:8000/api/telemetry');
        if (res.data?.processes) {
          setAlerts(res.data.processes.length);
          setHighRisk(res.data.processes.filter((p: any) => p.risk_level === 'high').length);
        }
        setEndpoints(1); // Placeholder since we only fetch 1 host in current setup
      } catch (err) {
        console.error('Metrics fetch failed', err);
      }
    };
    fetchData();
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <MetricCard title="Total Alerts" value={alerts.toString()} icon={<Activity />} />
      <MetricCard title="Active Endpoints" value={endpoints.toString()} icon={<Server />} />
      <MetricCard title="High Risk Processes" value={highRisk.toString()} icon={<ShieldAlert />} />
    </div>
  );
}
