import React, { useEffect, useState } from 'react';
import { Search } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Input } from '@/components/ui/input';

interface IOC {
  type: string;
  value: string;
  threat: string;
  date: string;
}

export default function IOCViewerPanel() {
  const [search, setSearch] = useState('');
  const [data, setData] = useState<IOC[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('http://localhost:8000/api/iocs')
      .then(res => {
        if (!res.ok) throw new Error('Failed to fetch IOCs');
        return res.json();
      })
      .then(setData)
      .catch(err => setError(err.message))
      .finally(() => setLoading(false));
  }, []);

  const filtered = data.filter(ioc =>
    ioc.value.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">🧪 IOC Viewer</h2>
        <div className="relative w-64">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
          <Input
            placeholder="Search IOCs..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-9"
          />
        </div>
      </div>

      {loading ? (
        <p className="text-sm text-gray-500">Loading IOCs...</p>
      ) : error ? (
        <p className="text-sm text-red-500">⚠️ {error}</p>
      ) : (
        <>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead>Value</TableHead>
                <TableHead>Threat</TableHead>
                <TableHead>Date</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filtered.map((ioc, i) => (
                <TableRow key={i}>
                  <TableCell>{ioc.type}</TableCell>
                  <TableCell className="font-mono">{ioc.value}</TableCell>
                  <TableCell>{ioc.threat}</TableCell>
                  <TableCell>{ioc.date}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {filtered.length === 0 && (
            <p className="text-sm text-gray-500 mt-4">No matching IOCs found.</p>
          )}
        </>
      )}
    </div>
  );
}

