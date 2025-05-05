import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

interface Explanation {
  feature: string;
  weight: number;
}

interface ExplainablePanelProps {
  selectedAlert: any | null;
}

export default function ExplainablePanel({ selectedAlert }: ExplainablePanelProps) {
  const [explanation, setExplanation] = useState<Explanation[]>([]);

  const fetchExplanation = async () => {
    if (!selectedAlert) return;
    try {
      const res = await axios.get(`/api/explain`);
      setExplanation(res.data);
    } catch (err) {
      console.error('Failed to fetch explanation:', err);
    }
  };

  useEffect(() => {
    fetchExplanation();
  }, [selectedAlert]);

  if (!selectedAlert) return null;

  return (
    <Card className="mb-8 bg-white shadow-xl p-4">
      <h2 className="text-xl font-semibold mb-4">🔎 Explainable SOC Panel</h2>

      <div className="mb-4 text-sm text-gray-700">
        <p><strong>Process:</strong> {selectedAlert.name}</p>
        <p><strong>Hostname:</strong> {selectedAlert.hostname}</p>
        <p><strong>Timestamp:</strong> {new Date(selectedAlert.timestamp).toLocaleString()}</p>
        <p><strong>Risk Score:</strong> {selectedAlert.risk_score}</p>
      </div>

      <Button className="mb-4" onClick={fetchExplanation}>Refresh Explanation</Button>

      <ul className="space-y-2 text-sm">
        {explanation.map((item, idx) => (
          <li key={idx} className="flex justify-between">
            <span className="font-medium">{item.feature}</span>
            <span className={`font-mono ${item.weight > 0 ? 'text-green-600' : 'text-red-600'}`}>
              {item.weight.toFixed(3)}
            </span>
          </li>
        ))}
      </ul>
    </Card>
  );
}
