import React, { useEffect, useState } from 'react';

interface Agent {
  hostname: string;
  pubkey: string;
  last_seen: string;
  binary_hash: string;
  policy_hash: string;
  attestation_passed: boolean;
  trust_score?: number;
}

export default function AgentRegistryPanel() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/agent')
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json() as Promise<Agent[]>;
      })
      .then(setAgents)
      .catch(err => setError(err.message));
  }, []);

  if (error) {
    return <div className="p-4 text-red-500">Error loading agents: {error}</div>;
  }

  return (
    <div className="mb-6 p-4 bg-white rounded-xl shadow-md">
      <h2 className="text-xl font-semibold mb-4">Registered Agents</h2>
      {agents.length === 0 ? (
        <p className="text-gray-500">No agents registered yet.</p>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm text-left text-gray-900 border border-gray-200">
            <thead className="bg-gray-100 text-xs uppercase">
              <tr>
                <th className="px-4 py-2">Hostname</th>
                <th className="px-4 py-2">Last Seen</th>
                <th className="px-4 py-2">Attested</th>
                <th className="px-4 py-2">Trust Score</th>
                <th className="px-4 py-2">Policy Hash</th>
              </tr>
            </thead>
            <tbody>
              {agents.map(agent => (
                <tr key={agent.hostname} className="border-t">
                  <td className="px-4 py-2">{agent.hostname}</td>
                  <td className="px-4 py-2">{new Date(agent.last_seen).toLocaleString()}</td>
                  <td className="px-4 py-2">
                    {agent.attestation_passed ? '✅' : '❌'}
                  </td>
                  <td className="px-4 py-2">
                    <span
                      className={`px-2 py-1 rounded-full text-xs font-semibold ${
                        agent.trust_score! >= 90
                          ? 'bg-green-100 text-green-800'
                          : agent.trust_score! >= 70
                          ? 'bg-yellow-100 text-yellow-800'
                          : 'bg-red-100 text-red-800'
                      }`}
                    >
                      {agent.trust_score?.toFixed(1)}
                    </span>
                  </td>
                  <td className="px-4 py-2 font-mono text-xs text-gray-700">
                    {agent.policy_hash.slice(0, 12)}...
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
