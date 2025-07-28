import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Bar } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Tooltip,
  Legend,
} from 'chart.js';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';

ChartJS.register(CategoryScale, LinearScale, BarElement, Tooltip, Legend);

interface ExplanationEntry {
  feature: string;
  weight: number;
}

interface ExplainPanelProps {
  selectedAlert: {
    name: string;
    hostname: string;
    timestamp: number;
    risk_score: number;
  } | null;
  features: number[];
  featureNames: string[];
}

export default function ExplainPanel({ selectedAlert, features, featureNames }: ExplainPanelProps) {
  const [explanation, setExplanation] = useState<ExplanationEntry[]>([]);

  const fetchExplanation = async () => {
    try {
      const res = await axios.post('/api/explain', {
        features,
        feature_names: featureNames,
      });
      setExplanation(res.data.explanation);
    } catch (err) {
      console.error('Failed to load explanation', err);
    }
  };

  useEffect(() => {
    fetchExplanation();
  }, [features, featureNames]);

  return (
    <Card className="mb-8 p-6 bg-white shadow">
      <h2 className="text-xl font-semibold mb-4">🔎 Explainable SOC Panel</h2>

      {selectedAlert && (
        <div className="mb-4 text-sm text-gray-700">
          <p><strong>Process:</strong> {selectedAlert.name}</p>
          <p><strong>Hostname:</strong> {selectedAlert.hostname}</p>
          <p><strong>Timestamp:</strong> {new Date(selectedAlert.timestamp).toLocaleString()}</p>
          <p><strong>Risk Score:</strong> {selectedAlert.risk_score}</p>
        </div>
      )}

      <Button className="mb-4" onClick={fetchExplanation}>Refresh Explanation</Button>

      {Array.isArray(explanation) && explanation.length > 0 ? (
        <Bar
          data={{
            labels: explanation.map(e => e.feature),
            datasets: [
              {
                label: 'Feature Impact',
                data: explanation.map(e => e.weight),
                backgroundColor: 'rgba(59, 130, 246, 0.6)',
              },
            ],
          }}
          options={{
            responsive: true,
            plugins: { legend: { display: false } },
            scales: {
              y: { beginAtZero: true },
            },
          }}
        />
      ) : (
        <p className="text-sm text-gray-500">No explanation data available.</p>
      )}
    </Card>
  );
}
