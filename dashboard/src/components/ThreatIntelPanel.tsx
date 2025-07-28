// src/components/ThreatIntelPanel.tsx
import React from 'react';
import { Shield, Globe2, Flame } from 'lucide-react';

const intel = [
  {
    id: 'ioc-01',
    title: '⚔️ APT29 C2 Infrastructure Discovered',
    type: 'C2 Server',
    detail: 'New command and control nodes linked to CozyBear found in Eastern Europe.',
    source: 'Mandiant',
  },
  {
    id: 'ioc-02',
    title: '🔥 Zero-Day in PDF Renderers Exploited in the Wild',
    type: 'Exploit',
    detail: 'Threat actors leveraging malformed annotations in embedded PDFs.',
    source: 'Google TAG',
  },
  {
    id: 'ioc-03',
    title: '🌐 Phishing Kits Using Azure Front Door',
    type: 'Phishing',
    detail: 'Actors bypassing email filters by hosting on Microsoft infrastructure.',
    source: 'Proofpoint',
  },
];

const getColor = (type: string) => {
  switch (type) {
    case 'C2 Server': return 'bg-red-100 text-red-800';
    case 'Exploit': return 'bg-yellow-100 text-yellow-800';
    case 'Phishing': return 'bg-blue-100 text-blue-800';
    default: return 'bg-gray-100 text-gray-800';
  }
};

export default function ThreatIntelPanel() {
  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
        <Globe2 className="w-5 h-5" /> Threat Intelligence Bulletin
      </h2>
      <div className="space-y-4">
        {intel.map(item => (
          <div key={item.id} className={`p-4 border-l-4 ${getColor(item.type)} rounded`}>
            <h3 className="text-sm font-semibold">{item.title}</h3>
            <p className="text-xs mt-1 text-gray-700">{item.detail}</p>
            <p className="text-xs mt-1 text-gray-500 italic">Source: {item.source}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
