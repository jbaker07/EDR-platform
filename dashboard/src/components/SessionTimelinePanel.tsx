import React, { useEffect, useState } from 'react';
import { Clock, Activity, Tag, ShieldX } from 'lucide-react';

interface TimelineEvent {
  timestamp: string;
  type: 'execution' | 'risk_update' | 'tagged' | 'terminated';
  description: string;
}

const mockEvents: TimelineEvent[] = [
  {
    timestamp: '2025-04-23T12:00:00Z',
    type: 'execution',
    description: 'Process started: chrome.exe (PID 1234)',
  },
  {
    timestamp: '2025-04-23T12:01:30Z',
    type: 'risk_update',
    description: 'Risk score updated: 0.75 → 0.91',
  },
  {
    timestamp: '2025-04-23T12:02:10Z',
    type: 'tagged',
    description: 'Process tagged for review by analyst',
  },
  {
    timestamp: '2025-04-23T12:03:00Z',
    type: 'terminated',
    description: 'Termination signal sent to process',
  },
];

const getIcon = (type: TimelineEvent['type']) => {
  switch (type) {
    case 'execution':
      return <Activity className="w-4 h-4 text-blue-600" />;
    case 'risk_update':
      return <Clock className="w-4 h-4 text-orange-500" />;
    case 'tagged':
      return <Tag className="w-4 h-4 text-yellow-600" />;
    case 'terminated':
      return <ShieldX className="w-4 h-4 text-red-600" />;
  }
};

export default function SessionTimelinePanel() {
  const [events, setEvents] = useState<TimelineEvent[]>([]);

  useEffect(() => {
    // TODO: Replace with live API pull when ready
    setEvents(mockEvents);
  }, []);

  return (
    <div className="bg-white rounded-xl shadow p-4 max-h-96 overflow-y-auto">
      <h2 className="text-lg font-semibold mb-3">🕒 Session Timeline</h2>
      <ul className="space-y-3 text-sm">
        {events.map((event, i) => (
          <li key={i} className="flex items-start gap-3">
            <div className="mt-1">{getIcon(event.type)}</div>
            <div>
              <p className="text-gray-800 font-medium">{event.description}</p>
              <p className="text-gray-500 text-xs">
                {new Date(event.timestamp).toLocaleString()}
              </p>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}