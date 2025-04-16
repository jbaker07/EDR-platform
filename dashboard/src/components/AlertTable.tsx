import React, { useState, useMemo } from 'react';
import Fuse from 'fuse.js';

interface Alert {
  id: number;
  agent: string;
  pid: number;
  processName: string;
  command: string;
  severity: 'Low' | 'Medium' | 'High';
  timestamp: string;  // ISO timestamp string
}

interface AlertTableProps {
  alerts: Alert[];
  onRowClick: (alert: Alert) => void;
}

const AlertTable: React.FC<AlertTableProps> = ({ alerts, onRowClick }) => {
  const [searchQuery, setSearchQuery] = useState('');
  const [severityFilter, setSeverityFilter] = useState<'All' | 'Low' | 'Medium' | 'High'>('All');
  const [sortColumn, setSortColumn] = useState<keyof Alert | null>(null);
  const [sortAsc, setSortAsc] = useState(true);

  // Memoize filtered + searched + sorted results for performance
  const filteredAlerts = useMemo(() => {
    let data = alerts;
    // Filter by severity if not "All"
    if (severityFilter !== 'All') {
      data = data.filter(alert => alert.severity === severityFilter);
    }
    // Fuzzy search by agent, processName, or command
    if (searchQuery.trim() !== '') {
      const fuse = new Fuse(data, { keys: ['agent', 'processName', 'command'], threshold: 0.3 });
      const results = fuse.search(searchQuery);
      data = results.map(r => r.item);
    }
    // Sort data if a sort column is set
    if (sortColumn) {
      data = [...data]; // clone to avoid mutating original
      data.sort((a, b) => {
        let valA = a[sortColumn];
        let valB = b[sortColumn];
        // For timestamp, compare as dates
        if (sortColumn === 'timestamp') {
            const valA = new Date(a.timestamp).getTime();
            const valB = new Date(b.timestamp).getTime();
            return valA - valB;            
        }
        // For severity, define a custom order High > Medium > Low
        if (sortColumn === 'severity') {
          const order = { 'High': 3, 'Medium': 2, 'Low': 1 };
          return (order[a.severity] - order[b.severity]) * (sortAsc ? 1 : -1);
        }
        // Default string/number comparison
        if (valA < valB) return sortAsc ? -1 : 1;
        if (valA > valB) return sortAsc ? 1 : -1;
        return 0;
      });
    }
    return data;
  }, [alerts, severityFilter, searchQuery, sortColumn, sortAsc]);

  const onSortClick = (col: keyof Alert) => {
    if (sortColumn === col) {
      // toggle sort direction
      setSortAsc(!sortAsc);
    } else {
      setSortColumn(col);
      setSortAsc(true);
    }
  };

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  const handleFilterChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSeverityFilter(e.target.value as any);
  };

  // Utility to get styling for severity badge
  const severityClasses = (sev: string) => {
    switch (sev) {
      case 'High':   return 'bg-red-100 text-red-800';
      case 'Medium': return 'bg-yellow-100 text-yellow-800';
      case 'Low':    return 'bg-green-100 text-green-800';
      default:       return '';
    }
  };

  return (
    <div className="bg-white rounded shadow p-4">
      {/* Table Controls: Search and Filter */}
      <div className="flex flex-col md:flex-row md:items-center md:justify-between mb-3">
        <input
          type="text"
          value={searchQuery}
          onChange={handleSearchChange}
          placeholder="Search alerts..."
          className="w-full md:w-1/3 mb-2 md:mb-0 px-3 py-2 border rounded"
        />
        <div className="flex items-center space-x-2">
          <label htmlFor="severityFilter" className="font-medium">Severity:</label>
          <select 
            id="severityFilter" 
            value={severityFilter} 
            onChange={handleFilterChange}
            className="px-2 py-1 border rounded"
          >
            <option value="All">All</option>
            <option value="High">High</option>
            <option value="Medium">Medium</option>
            <option value="Low">Low</option>
          </select>
          {/* Export CSV Button */}
          <button 
            onClick={() => exportToCSV(alerts)} 
            className="ml-4 px-3 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm"
          >
            Export CSV
          </button>
        </div>
      </div>

      {/* Alerts Table */}
      <div className="overflow-x-auto">
        <table className="min-w-full text-sm text-left">
          <thead className="bg-gray-100 text-gray-800">
            <tr>
              {['Agent', 'PID', 'Process Name', 'Command', 'Severity', 'Timestamp'].map((col, idx) => {
                // Determine corresponding key in Alert
                const colKey = (col === 'Process Name' ? 'processName' : col.toLowerCase()) as keyof Alert;
                return (
                  <th 
                    key={idx} 
                    onClick={() => onSortClick(colKey)} 
                    className="py-2 px-3 font-semibold cursor-pointer select-none"
                  >
                    {col}
                    {sortColumn === colKey && (
                      <span className="ml-1 text-xs">
                        {sortAsc ? '▲' : '▼'}
                      </span>
                    )}
                  </th>
                );
              })}
            </tr>
          </thead>
          <tbody>
            {filteredAlerts.map(alert => (
              <tr 
                key={alert.id} 
                onClick={() => onRowClick(alert)} 
                className="hover:bg-gray-50 cursor-pointer"
              >
                <td className="py-1 px-3">{alert.agent}</td>
                <td className="py-1 px-3">{alert.pid}</td>
                <td className="py-1 px-3">{alert.processName}</td>
                <td className="py-1 px-3">{alert.command}</td>
                <td className="py-1 px-3">
                  <span className={`text-xs font-semibold px-2 py-0.5 rounded ${severityClasses(alert.severity)}`}>
                    {alert.severity}
                  </span>
                </td>
                <td className="py-1 px-3">{new Date(alert.timestamp).toLocaleString()}</td>
              </tr>
            ))}
            {filteredAlerts.length === 0 && (
              <tr><td className="py-2 px-3 italic text-gray-500" colSpan={6}>No matching alerts</td></tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

// Helper to export alerts data to CSV
function exportToCSV(alerts: Alert[]) {
  const header = "Agent,PID,Process Name,Command,Severity,Timestamp\n";
  const rows = alerts.map(a => 
    `${a.agent},${a.pid},${a.processName},${a.command},${a.severity},${a.timestamp}`
  );
  const csvContent = header + rows.join("\n");
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.setAttribute('download', 'telemetry_alerts.csv');
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

export default AlertTable;
