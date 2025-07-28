import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from '@/components/ui/table';
import { Input } from '@/components/ui/input';

interface FeedbackEntry {
  timestamp: string;
  source: 'analyst' | 'automation';
  type: 'false_positive' | 'false_negative' | 'suggestion';
  message: string;
  related_pid?: number;
  hostname?: string;
}

export default function FeedbackLogPanel() {
  const [entries, setEntries] = useState<FeedbackEntry[]>([]);
  const [search, setSearch] = useState('');

  useEffect(() => {
    axios.get('/api/feedback').then((res) => setEntries(res.data));
  }, []);

  const filtered = entries.filter(
    (entry) =>
      entry.message.toLowerCase().includes(search.toLowerCase()) ||
      entry.hostname?.toLowerCase().includes(search.toLowerCase()) ||
      String(entry.related_pid).includes(search)
  );

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">📋 Feedback Logs</h2>
      <Input
        placeholder="Search by message, PID, or hostname..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        className="mb-4"
      />
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Timestamp</TableHead>
            <TableHead>Source</TableHead>
            <TableHead>Type</TableHead>
            <TableHead>Message</TableHead>
            <TableHead>PID</TableHead>
            <TableHead>Hostname</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filtered.map((entry, i) => (
            <TableRow key={i}>
              <TableCell>{new Date(entry.timestamp).toLocaleString()}</TableCell>
              <TableCell>{entry.source}</TableCell>
              <TableCell>{entry.type}</TableCell>
              <TableCell>{entry.message}</TableCell>
              <TableCell>{entry.related_pid ?? '-'}</TableCell>
              <TableCell>{entry.hostname ?? '-'}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {filtered.length === 0 && (
        <p className="text-sm text-gray-500 mt-4">No feedback entries found.</p>
      )}
    </div>
  );
}