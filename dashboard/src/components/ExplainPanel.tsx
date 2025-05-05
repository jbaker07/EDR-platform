// src/components/ExplainPanel.tsx
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

ChartJS.register(CategoryScale, LinearScale, BarElement, Tooltip, Legend);

interface ExplanationProps {
  features: number[];
  featureNames: string[];
}

interface ExplanationEntry {
  feature: string;
  weight: number;
}

export default function ExplainPanel({ features, featureNames }: ExplanationProps) {
  const [explanation, setExplanation] = useState<ExplanationEntry[]>([]);

  useEffect(() => {
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
    fetchExplanation();
  }, [features, featureNames]);

  return (
    <Card className="mb-6">
      <CardContent>
        <h2 className="text-lg font-semibold mb-4">📊 Feature Attribution (XAI)</h2>
        {explanation.length > 0 ? (
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
          <p className="text-sm text-gray-500">Loading explanation...</p>
        )}
      </CardContent>
    </Card>
  );
}