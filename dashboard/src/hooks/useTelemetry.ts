// src/hooks/useTelemetry.ts

import { useEffect, useState } from "react";

export interface TelemetryEntry {
  id: number;
  agent_id: string;
  timestamp: string;
  data: any;
}

export const useTelemetry = () => {
  const [data, setData] = useState<TelemetryEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch("http://localhost:8000/api/telemetry/all")
      .then((res) => res.json())
      .then((json) => {
        setData(json);
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, []);

  return { data, loading };
};
