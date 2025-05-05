import React from 'react';

const playbooks = [
  {
    id: 'pb-001',
    name: 'High CPU Process Termination',
    steps: [
      'Detect process with CPU > 90%',
      'Score process risk using model',
      'If score > 0.8 → tag as threat',
      'Trigger termination via SOAR agent',
    ],
    severity: 'high',
  },
  {
    id: 'pb-002',
    name: 'Unauthorized USB Insertion',
    steps: [
      'Detect new USB mount event',
      'Log serial ID and mount path',
      'Check against allowlist',
      'If unknown → trigger alert and lock screen',
    ],
    severity: 'medium',
  },
];

const getSeverityColor = (sev: string) => {
  return sev === 'high'
    ? 'bg-red-100 text-red-800'
    : sev === 'medium'
    ? 'bg-yellow-100 text-yellow-800'
    : 'bg-green-100 text-green-800';
};

export default function PlaybookPanel() {
  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">🧾 Response Playbooks</h2>
      <div className="grid gap-4 sm:grid-cols-1 md:grid-cols-2 xl:grid-cols-3">
        {playbooks.map(pb => (
          <div
            key={pb.id}
            className="border border-gray-200 p-4 rounded-lg bg-gray-50 hover:shadow-lg transition duration-200"
          >
            <div className="flex justify-between items-center mb-2">
              <h3 className="font-bold text-md">{pb.name}</h3>
              <span className={`text-xs px-2 py-1 font-semibold rounded ${getSeverityColor(pb.severity)}`}>
                {pb.severity.toUpperCase()}
              </span>
            </div>
            <ol className="text-sm text-gray-700 list-decimal ml-5 space-y-1">
              {pb.steps.map((step, i) => (
                <li key={i}>{step}</li>
              ))}
            </ol>
            <div className="text-right mt-3">
              <button
                className="text-sm px-3 py-1 bg-blue-100 text-blue-800 rounded hover:bg-blue-200"
                onClick={() => alert(`Simulating: ${pb.name}`)}
              >
                ▶️ Simulate
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

