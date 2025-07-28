import React, { useEffect, useState } from 'react';
import { ExplanationPayload } from '@/types/Explanation';

interface TimelineEvent {
  id: string;
  timestamp: string;
  description: string;
}

interface TimelinePanelProps {
  events: TimelineEvent[];
}

export default function TimelinePanel({ events }: TimelinePanelProps) {
  const [explainMode, setExplainMode] = useState(false);
  const [explanations, setExplanations] = useState<Record<string, ExplanationPayload>>({});

  const fetchExplanation = async (eventId: string) => {
    try {
      const res = await fetch(`/api/explain/timeline_event?event_id=${eventId}`);
      const data: ExplanationPayload = await res.json();
      setExplanations(prev => ({ ...prev, [eventId]: data }));
    } catch (err) {
      console.error(`Failed to fetch explanation for ${eventId}`, err);
    }
  };

  const toggleExplainMode = () => {
    setExplainMode(prev => !prev);
    if (!explainMode) {
      // Load all explanations in parallel
      events.forEach(e => fetchExplanation(e.id));
    }
  };

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">🧠 Forensic Timeline</h2>
        <button
          onClick={toggleExplainMode}
          className="px-4 py-2 rounded bg-indigo-600 text-white hover:bg-indigo-700"
        >
          {explainMode ? 'Hide Explain Mode' : 'Explain Mode'}
        </button>
      </div>

      <ul className="space-y-3">
        {events.map((event) => (
          <li key={event.id} className="bg-white shadow rounded p-4">
            <div className="text-sm text-gray-600">{event.timestamp}</div>
            <div className="text-md font-medium">{event.description}</div>

            {explainMode && explanations[event.id] && (
              <div className="mt-3 border-t pt-3 text-sm space-y-2">
                <div>
                  <strong>Top SHAP Features:</strong>
                  <ul className="list-disc ml-5">
                    {explanations[event.id].shap_reasoning.top_contributors.map((feat, i) => (
                      <li key={i}>
                        {feat.feature}: {feat.impact.toFixed(4)}
                      </li>
                    ))}
                  </ul>
                </div>

                <div>
                  <strong>Trust Score Change:</strong> {explanations[event.id].trust_score_reasoning.delta}
                  <ul className="list-disc ml-5">
                    {explanations[event.id].trust_score_reasoning.reasons.map((reason, i) => (
                      <li key={i}>{reason}</li>
                    ))}
                  </ul>
                </div>

                <div>
                  <strong>GNN Context:</strong>
                  <div>Node: {explanations[event.id].gnn_context.node}</div>
                  <div>Parents: {explanations[event.id].gnn_context.causal_parents.join(', ')}</div>
                  <div>Neighbors: {explanations[event.id].gnn_context.related_nodes.join(', ')}</div>
                </div>
              </div>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
