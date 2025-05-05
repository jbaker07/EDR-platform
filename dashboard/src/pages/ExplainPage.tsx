// src/pages/ExplainPage.tsx
import React from 'react';
import ExplainPanel from '@/components/ExplainPanel';

export default function ExplainPage() {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-semibold mb-4">🔍 Model Explainability</h1>
      <ExplainPanel
        features={[0.3, 512]}
        featureNames={['cpu_usage', 'memory']}
      />
    </div>
  );
}
