import React from 'react';

interface Endpoint {
  id: string;
  hostname: string;
  ip: string;
  risk: number; // from 0 to 100
}

const mockEndpoints: Endpoint[] = [
  { id: '1', hostname: 'host-a', ip: '192.168.1.10', risk: 15 },
  { id: '2', hostname: 'host-b', ip: '192.168.1.20', risk: 65 },
  { id: '3', hostname: 'host-c', ip: '192.168.1.30', risk: 92 },
  { id: '4', hostname: 'host-d', ip: '192.168.1.40', risk: 35 },
  { id: '5', hostname: 'host-e', ip: '192.168.1.50', risk: 80 },
  { id: '6', hostname: 'host-f', ip: '192.168.1.60', risk: 5 },
];

const getColor = (risk: number) => {
  if (risk > 80) return 'bg-red-600';
  if (risk > 50) return 'bg-yellow-400';
  if (risk > 20) return 'bg-blue-400';
  return 'bg-green-500';
};

export default function EndpointHeatmap() {
  return (
    <div className="p-6 bg-white rounded-xl shadow mb-10">
      <h2 className="text-xl font-semibold mb-4">🌐 Endpoint Risk Heatmap</h2>
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        {mockEndpoints.map(ep => (
          <div
            key={ep.id}
            className={`rounded-lg p-3 text-center text-white shadow ${getColor(ep.risk)}`}
          >
            <p className="font-semibold text-sm">{ep.hostname}</p>
            <p className="text-xs">{ep.ip}</p>
            <p className="mt-1 text-lg font-bold">{ep.risk}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
