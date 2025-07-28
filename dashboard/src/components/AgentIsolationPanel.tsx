// src/components/AgentIsolationPanel.tsx
import React, { useState } from 'react';

interface Agent {
  id: string;
  hostname: string;
  ip: string;
  status: 'online' | 'offline';
  isolated: boolean;
}

const mockAgents: Agent[] = [
  { id: 'a-001', hostname: 'alpha', ip: '192.168.1.10', status: 'online', isolated: false },
  { id: 'a-002', hostname: 'bravo', ip: '192.168.1.11', status: 'online', isolated: true },
  { id: 'a-003', hostname: 'charlie', ip: '192.168.1.12', status: 'offline', isolated: false },
];

export default function AgentIsolationPanel() {
  const [agents, setAgents] = useState(mockAgents);

  const toggleIsolation = (id: string) => {
    setAgents(prev =>
      prev.map(agent =>
        agent.id === id ? { ...agent, isolated: !agent.isolated } : agent
      )
    );
  };

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">🔒 Agent Isolation Control</h2>
      <div className="space-y-4">
        {agents.map(agent => (
          <div
            key={agent.id}
            className={`border rounded-lg p-4 flex justify-between items-center ${
              agent.status === 'offline' ? 'opacity-60' : ''
            }`}
          >
            <div>
              <h3 className="text-lg font-bold">{agent.hostname}</h3>
              <p className="text-sm text-gray-600">IP: {agent.ip}</p>
              <p className="text-sm text-gray-500">Status: {agent.status.toUpperCase()}</p>
            </div>
            <button
              disabled={agent.status === 'offline'}
              onClick={() => toggleIsolation(agent.id)}
              className={`px-4 py-2 rounded text-sm font-semibold ${
                agent.isolated
                  ? 'bg-green-100 text-green-800 hover:bg-green-200'
                  : 'bg-red-100 text-red-800 hover:bg-red-200'
              }`}
            >
              {agent.isolated ? 'Restore Connectivity' : 'Isolate Agent'}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
