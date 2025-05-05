// src/components/AgentActivityFeed.tsx
import React, { useEffect, useState } from 'react';
import { TerminalSquare, Usb, FileText, Zap } from 'lucide-react';

interface ActivityEvent {
  timestamp: string;
  type: 'process' | 'file' | 'usb' | 'custom';
  description: string;
  agent: string;
}

const icons = {
  process: <TerminalSquare className="text-blue-600 w-4 h-4" />,
  file: <FileText className="text-yellow-600 w-4 h-4" />,
  usb: <Usb className="text-green-600 w-4 h-4" />,
  custom: <Zap className="text-purple-600 w-4 h-4" />,
};

const mockEvents: ActivityEvent[] = [
  { timestamp: '12:34:56', type: 'process', description: 'Started notepad.exe', agent: 'host-a' },
  { timestamp: '12:35:02', type: 'file', description: 'Modified secrets.txt', agent: 'host-b' },
  { timestamp: '12:36:10', type: 'usb', description: 'Inserted USB Drive', agent: 'host-c' },
];

export default function AgentActivityFeed() {
  const [events, setEvents] = useState<ActivityEvent[]>([]);

  useEffect(() => {
    setEvents(mockEvents); // Replace with fetch from backend later
  }, []);

  return (
    <div className="bg-white p-4 rounded-xl shadow-md h-96 overflow-y-auto">
      <h2 className="text-lg font-semibold mb-3">📡 Agent Activity Feed</h2>
      <ul className="space-y-3">
        {events.map((e, idx) => (
          <li key={idx} className="flex items-start gap-3 text-sm">
            <div className="pt-1">{icons[e.type]}</div>
            <div>
              <p className="text-gray-800">{e.description}</p>
              <p className="text-gray-500 text-xs">{e.timestamp} • {e.agent}</p>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
