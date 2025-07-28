import React, { useEffect, useState } from 'react';
import axios from 'axios';
import EndpointModal from './EndpointModal'; // ✅ Import Modal Component

interface Endpoint {
  id: string;
  hostname: string;
  ip: string;
  status: 'online' | 'offline';
  riskLevel: 'low' | 'medium' | 'high';
}

const fallbackEndpoints: Endpoint[] = [
  { id: '1', hostname: 'host-a', ip: '192.168.0.12', status: 'online', riskLevel: 'low' },
  { id: '2', hostname: 'host-b', ip: '192.168.0.15', status: 'online', riskLevel: 'high' },
  { id: '3', hostname: 'host-c', ip: '192.168.0.23', status: 'offline', riskLevel: 'medium' },
];

const EndpointGrid: React.FC = () => {
  const [endpoints, setEndpoints] = useState<Endpoint[]>(fallbackEndpoints);
  const [selected, setSelected] = useState<Endpoint | null>(null); // ✅ Modal state

  // Toggle below to false if backend route isn't ready yet
  const USE_BACKEND = false;

  useEffect(() => {
    if (USE_BACKEND) {
      axios.get('http://localhost:8000/telemetry/agents').then(res => {
        setEndpoints(res.data);
      }).catch(() => {
        setEndpoints(fallbackEndpoints);
      });
    }
  }, []);

  return (
    <>
      <div className="bg-white p-6 rounded-2xl shadow-xl col-span-1 md:col-span-2 xl:col-span-3">
        <h2 className="text-xl font-semibold mb-4">🖥️ Endpoint Overview</h2>
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2 xl:grid-cols-3">
          {endpoints.length === 0 ? (
            <p className="text-gray-500 text-sm">No endpoints available.</p>
          ) : (
            endpoints.map((ep) => (
              <div
                key={ep.id}
                onClick={() => setSelected(ep)} // ✅ Open modal on click
                className="bg-gray-50 p-4 rounded-lg border shadow hover:shadow-lg transition cursor-pointer"
              >
                <div className="flex justify-between items-center mb-2">
                  <h3 className="font-bold text-lg">{ep.hostname}</h3>
                  <span
                    className={`px-2 py-1 text-xs font-semibold rounded ${
                      ep.status === 'online' ? 'bg-green-500' : 'bg-gray-400'
                    } text-white`}
                  >
                    {ep.status.toUpperCase()}
                  </span>
                </div>
                <p className="text-sm text-gray-700">IP: {ep.ip}</p>
                <p className="text-sm mt-2">
                  Risk Level:{' '}
                  <span
                    className={`font-semibold ${
                      ep.riskLevel === 'high'
                        ? 'text-red-600'
                        : ep.riskLevel === 'medium'
                        ? 'text-yellow-500'
                        : 'text-green-600'
                    }`}
                  >
                    {ep.riskLevel.toUpperCase()}
                  </span>
                </p>
              </div>
            ))
          )}
        </div>
      </div>

      {/* ✅ Modal Rendering */}
      {selected && <EndpointModal endpoint={selected} onClose={() => setSelected(null)} />}
    </>
  );
};

export default EndpointGrid;
