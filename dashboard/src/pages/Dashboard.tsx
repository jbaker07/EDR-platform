import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  LineElement,
  PointElement,
  LinearScale,
  Title,
  CategoryScale,
  Tooltip,
  Legend
} from 'chart.js';
import Fuse from 'fuse.js';

ChartJS.register(LineElement, PointElement, LinearScale, Title, CategoryScale, Tooltip, Legend);

interface ProcessEntry {
  agent_id: string;
  pid: number;
  name: string;
  command: string;
  severity: string;
  timestamp: string;
}

const getBadgeColor = (severity: string) => {
  switch (severity.toLowerCase()) {
    case 'high':
      return 'bg-red-100 text-red-800';
    case 'medium':
      return 'bg-yellow-100 text-yellow-800';
    case 'low':
    default:
      return 'bg-green-100 text-green-800';
  }
};

const Dashboard: React.FC = () => {
  const [data, setData] = useState<ProcessEntry[]>([]);
  const [query, setQuery] = useState('');
  const [filtered, setFiltered] = useState<ProcessEntry[]>([]);
  const [selected, setSelected] = useState<ProcessEntry | null>(null);
  const [live, setLive] = useState(true);
  const [severityFilter, setSeverityFilter] = useState<string>('all');
  const [selectedAgent, setSelectedAgent] = useState<string>('all');
  const [history, setHistory] = useState<ProcessEntry[]>([]);

  const fetchData = () => {
    fetch('http://localhost:8000/telemetry/all')
      .then(async res => {
        if (!res.ok) throw new Error('Failed to fetch data');
        return await
        res.json();
      })
      .catch(error => {
        console.error('Telemetry fetch error:', error);
        return [];
      })
      .then((newData: ProcessEntry[]) => {
        if (newData.length > 0) {
        setData(newData);
        setFiltered(newData);
        setHistory(prev => [...prev, ...newData]);
        }
      });
  };

  useEffect(() => {
    fetchData();
    let interval: ReturnType<typeof setInterval>;
    if (live) {
      interval = setInterval(fetchData, 5000);
    }
    return () => clearInterval(interval);
  }, [live]);

  useEffect(() => {
    const fuse = new Fuse(data, {
      keys: ['name', 'command', 'agent_id', 'severity'],
      threshold: 0.3,
    });
    let result = query.trim() ? fuse.search(query).map(r => r.item) : data;
    if (severityFilter !== 'all') {
      result = result.filter(entry => entry.severity.toLowerCase() === severityFilter);
    }
    if (selectedAgent !== 'all') {
      result = result.filter(entry => entry.agent_id === selectedAgent);
    }
    setFiltered(result);
  }, [query, data, severityFilter, selectedAgent]);

  const chartData = {
    labels: (filtered || []).map((entry, i) =>
      entry.timestamp ? new Date(entry.timestamp).toLocaleTimeString() : `Unknown-${i}`
    ),
    datasets: [
      {
        label: 'Alerts Over Time',
        data: (filtered || []).map((_entry, i) => i + 1),
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
      },
    ],
  };
  
  

  const chartOptions: any = {
    responsive: true,
    plugins: {
      legend: { position: 'top' },
      title: {
        display: true,
        text: 'Alerts Over Time',
      },
    },
  };
  

  const uniqueAgents = Array.from(new Set(data.map(entry => entry.agent_id)));

  const alertCorrelationMap: Record<string, number> = data.reduce((acc, curr) => {
    acc[curr.agent_id] = (acc[curr.agent_id] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  const getRecentAlerts = (agent_id: string) => {
    return history.filter(entry => entry.agent_id === agent_id).slice(-5);
  };

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <h1 className="text-4xl font-bold mb-8 text-center text-blue-700">📊 EDR Security Dashboard</h1>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-8">
        <div className="bg-white rounded-2xl p-4 shadow-md text-center">
          <p className="text-gray-500 text-sm">🛡️ Total Alerts</p>
          <p className="text-2xl font-bold text-blue-600">{filtered.length}</p>
        </div>
        <div className="bg-white rounded-2xl p-4 shadow-md text-center">
          <p className="text-gray-500 text-sm">🔥 High Severity</p>
          <p className="text-2xl font-bold text-red-500">{filtered.filter(e => e.severity.toLowerCase() === 'high').length}</p>
        </div>
        <div className="bg-white rounded-2xl p-4 shadow-md text-center">
          <p className="text-gray-500 text-sm">⚠️ Medium Severity</p>
          <p className="text-2xl font-bold text-yellow-500">{filtered.filter(e => e.severity.toLowerCase() === 'medium').length}</p>
        </div>
        <div className="bg-white rounded-2xl p-4 shadow-md text-center">
          <p className="text-gray-500 text-sm">✅ Low Severity</p>
          <p className="text-2xl font-bold text-green-500">{filtered.filter(e => e.severity.toLowerCase() === 'low').length}</p>
        </div>
      </div>

      {/* Severity Heatmap */}
      <div className="mb-6 grid grid-cols-2 md:grid-cols-4 gap-4">
        {uniqueAgents.map(agent => {
          const agentData = data.filter(d => d.agent_id === agent);
          const highCount = agentData.filter(e => e.severity.toLowerCase() === 'high').length;
          const mediumCount = agentData.filter(e => e.severity.toLowerCase() === 'medium').length;
          const lowCount = agentData.filter(e => e.severity.toLowerCase() === 'low').length;
          const recentAlerts = getRecentAlerts(agent);
          return (
            <div key={agent} className="rounded-xl p-4 shadow-md bg-white">
              <h3 className="text-blue-600 font-bold">{agent}</h3>
              <div className="flex justify-between text-sm mt-2">
                <span className="text-red-500">High: {highCount}</span>
                <span className="text-yellow-500">Med: {mediumCount}</span>
                <span className="text-green-500">Low: {lowCount}</span>
              </div>
              <div className="mt-4">
                <h4 className="text-sm font-semibold mb-1">Recent Alerts</h4>
                <ul className="text-xs text-gray-600 space-y-1">
                  {recentAlerts.map(alert => (
                    <li key={alert.timestamp + alert.pid}>
                      <span className={`px-2 py-1 rounded-full text-xs ${getBadgeColor(alert.severity)}`}>{alert.severity}</span> {alert.name}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          );
        })}
      </div>

      <div className="bg-white shadow rounded-xl p-6 mb-10">
  <h2 className="text-xl font-semibold mb-4">📉 Alerts Over Time</h2>
  {chartData.labels.length && chartData.datasets[0].data.length ? (
        <Line data={chartData} options={{ ...chartOptions, animation: false }} />
      ) : (
        <p className="text-gray-400 text-sm italic">Chart data is loading or unavailable.</p>
      )}
      
</div>


      {/* Placeholder for Alerts Table and Controls */}
      <div className="bg-white shadow rounded-xl p-6">
        <h2 className="text-xl font-semibold mb-4">📋 Alerts Table, Controls, and Correlation (coming next)</h2>
      </div>
    </div>
  );
};

export default Dashboard;