import React from 'react';
import { motion } from 'framer-motion';
import { Clock, Activity, Zap, ShieldCheck } from 'lucide-react';

const timeline = [
  {
    time: '12:00 PM',
    label: 'Agent Started',
    description: 'Agent initialized and started monitoring.',
    icon: <Clock className="w-4 h-4" />,
  },
  {
    time: '12:05 PM',
    label: 'High CPU Process Detected',
    description: 'PID 1234 exceeded CPU threshold.',
    icon: <Activity className="w-4 h-4" />,
  },
  {
    time: '12:07 PM',
    label: 'Playbook Triggered',
    description: 'High CPU playbook activated.',
    icon: <Zap className="w-4 h-4" />,
  },
  {
    time: '12:08 PM',
    label: 'Process Terminated',
    description: 'Process PID 1234 was safely terminated.',
    icon: <ShieldCheck className="w-4 h-4" />,
  },
];

export default function IncidentTimelinePanel() {
  return (
    <div className="bg-white p-6 rounded-xl shadow-xl mt-8">
      <h2 className="text-xl font-semibold mb-4">📅 Incident Timeline</h2>
      <div className="space-y-6 border-l-2 border-gray-200 pl-4">
        {timeline.map((item, idx) => (
          <motion.div
            key={idx}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: idx * 0.1 }}
            className="relative"
          >
            <div className="absolute -left-6 top-0 bg-white p-1 rounded-full border shadow">
              {item.icon}
            </div>
            <div className="ml-4">
              <p className="text-sm text-gray-600">{item.time}</p>
              <p className="font-semibold text-gray-900">{item.label}</p>
              <p className="text-sm text-gray-500">{item.description}</p>
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
}
