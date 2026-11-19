import React, { useEffect, useState } from 'react';
import axios from 'axios';
import EndpointModal from './EndpointModal';

interface Endpoint {
  id: string;
  hostname: string;
  ip: string;
  status: 'online' | 'offline';
  riskLevel: 'low' | 'medium' | 'high';
}

const EndpointGrid: React.FC = () => {
  const [endpoints, setEndpoints] = useState<Endpoint[]>([]);
  const [selected, setSelected] = useState<Endpoint | null>(null); // Modal state

  useEffect(() => {
    // GET /api/telemetry/agents in backend/app/routes/telemetry_router.py builds
    // this list from stored telemetry. It replaced three hard-coded fallback
    // hosts. The collector sends no IP address, so the grid says so.
    axios.get('/api/telemetry/agents')
      .then(res => setEndpoints(res.data.map((ep: Omit<Endpoint, 'ip'>) => ({ ...ep, ip: 'not collected' }))))
      .catch(err => console.error('Failed to load endpoints:', err));
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
                onClick={() => setSelected(ep)} // Open modal on click
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

      {/* Modal Rendering */}
      {selected && <EndpointModal endpoint={selected} onClose={() => setSelected(null)} />}
    </>
  );
};

export default EndpointGrid;
