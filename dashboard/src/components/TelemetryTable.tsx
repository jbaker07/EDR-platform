import React from 'react';

export function TelemetryTable() {
  const sampleData = [
    { pid: 1234, name: 'malicious.exe', severity: 'high' },
    { pid: 2345, name: 'explorer.exe', severity: 'low' },
    { pid: 3456, name: 'chrome.exe', severity: 'medium' },
  ];

  return (
    <div className="overflow-x-auto bg-white shadow-md rounded-2xl p-4">
      <h3 className="text-lg font-bold mb-4 text-gray-700">Live Processes</h3>
      <table className="min-w-full text-sm text-gray-700">
        <thead className="border-b font-medium bg-gray-50">
          <tr>
            <th className="text-left p-2">PID</th>
            <th className="text-left p-2">Name</th>
            <th className="text-left p-2">Severity</th>
          </tr>
        </thead>
        <tbody>
          {sampleData.map((entry, idx) => (
            <tr key={idx} className="border-t hover:bg-gray-100">
              <td className="p-2">{entry.pid}</td>
              <td className="p-2">{entry.name}</td>
              <td className={`p-2 font-semibold ${getSeverityColor(entry.severity)}`}>
                {entry.severity}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function getSeverityColor(severity: string) {
  switch (severity) {
    case 'high':
      return 'text-red-600';
    case 'medium':
      return 'text-yellow-500';
    case 'low':
      return 'text-green-600';
    default:
      return 'text-gray-500';
  }
}
