import React, { useState } from 'react';
import { scoreProcesses } from '../api/useScore';

export default function ScoreTest() {
  const [result, setResult] = useState<any>(null);

  const handleScore = async () => {
    const testData = [
      { pid: 123, memory: 512000, cpu_usage: 3.5 },
      { pid: 456, memory: 1024000, cpu_usage: 12.1 },
    ];
    const res = await scoreProcesses(testData);
    setResult(res);
  };

  return (
    <div className="p-4">
      <button
        className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
        onClick={handleScore}
      >
        Score Processes
      </button>
      {result && (
        <pre className="mt-4 bg-gray-100 p-3 rounded text-sm overflow-x-auto">
          {JSON.stringify(result, null, 2)}
        </pre>
      )}
    </div>
  );
}
