// NotificationBanner.tsx
import React from 'react';
import { Megaphone } from 'lucide-react';

export default function NotificationBanner() {
  return (
    <div className="bg-indigo-600 text-white px-6 py-2 text-sm flex items-center justify-between shadow-md rounded-md mb-4 animate-fade-in-down">
      <div className="flex items-center gap-2">
        <Megaphone size={16} className="text-white" />
        <span>📢 Heads-up: New red team simulation is live. Review playbooks.</span>
      </div>
      <button
        className="text-indigo-200 hover:text-white text-xs"
        onClick={() => alert('Opening playbook...')}
      >
        View Details →
      </button>
    </div>
  );
}
