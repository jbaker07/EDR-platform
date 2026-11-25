import { useEffect, useState } from 'react';
import { apiJson } from '@/api/client';
import { formatIsoTime } from '@/lib/utils';

interface Agent {
  hostname: string;
  pubkey: string;
  last_seen: string;
  binary_hash: string;
  policy_hash: string;
  attestation_passed: boolean;
}

// Agents that called POST /api/agent/register, as stored.
export default function AgentRegistryPanel() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    apiJson<Agent[]>('/api/agent')
      .then(setAgents)
      .catch((err) => setError(err.message));
  }, []);

  if (error) {
    return <div className="p-4 text-red-500">Error loading agents: {error}</div>;
  }

  return (
    <div className="p-6 bg-white rounded-xl shadow">
      <h2 className="text-xl font-semibold mb-4">Registered Agents</h2>
      {agents.length === 0 ? (
        <p className="text-gray-500 text-sm">No agents registered yet.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm text-left text-gray-900 border border-gray-200">
            <thead className="bg-gray-100 text-xs uppercase">
              <tr>
                <th className="px-4 py-2">Hostname</th>
                <th className="px-4 py-2">Last Seen</th>
                <th className="px-4 py-2">Attested</th>
                <th className="px-4 py-2">Binary Hash</th>
                <th className="px-4 py-2">Policy Hash</th>
              </tr>
            </thead>
            <tbody>
              {agents.map((agent) => (
                <tr key={agent.hostname} className="border-t">
                  <td className="px-4 py-2">{agent.hostname}</td>
                  <td className="px-4 py-2">{formatIsoTime(agent.last_seen)}</td>
                  <td className="px-4 py-2">{agent.attestation_passed ? 'yes' : 'no'}</td>
                  <td className="px-4 py-2 font-mono text-xs text-gray-700">{agent.binary_hash.slice(0, 12)}...</td>
                  <td className="px-4 py-2 font-mono text-xs text-gray-700">{agent.policy_hash.slice(0, 12)}...</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
