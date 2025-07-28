import React, { useEffect, useState } from 'react';
import ExplainPanel from '@/components/ExplainPanel';
import TimelinePanel from '@/components/TimelinePanel';

interface TimelineEvent {
  id: string;
  timestamp: string;
  description: string;
  features?: number[];
  featureNames?: string[];
}

export default function ExplainPage() {
  const [events, setEvents] = useState<TimelineEvent[]>([]);

  useEffect(() => {
    const fetchEvents = async () => {
      try {
        const res = await fetch('/api/telemetry/all');
        const data = await res.json();
        setEvents(data);
      } catch (err) {
        console.error('Failed to load timeline events', err);
      }
    };

    fetchEvents();
  }, []);

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">🔍 Explainability Overview</h1>
      <ExplainPanel
        selectedAlert={{
          name: "example_process",
          hostname: "localhost",
          timestamp: Date.now(),
          risk_score: 42,
        }}
        features={[0.3, 512]}
        featureNames={["cpu_usage", "memory"]}
      />
      <h2 className="text-xl font-semibold mt-8 mb-4">📅 Timeline of Events</h2>
      <TimelinePanel events={events} />
    </div>
  );
}
