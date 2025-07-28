import React, { useEffect, useState } from 'react';

interface ReplayStep {
  node_id: string;
  timestamp: string;
  event_type: string;
  shap_top: string;
  trust_delta: number;
}

interface Props {
  eventId: string;
}

const GNNReplay: React.FC<Props> = ({ eventId }) => {
  const [replay, setReplay] = useState<ReplayStep[]>([]);
  const [stepIndex, setStepIndex] = useState(0);
  const [playing, setPlaying] = useState(false);

  useEffect(() => {
    fetch(`/api/gnn/replay?event_id=${eventId}`)
      .then(res => res.json())
      .then(data => setReplay(data.replay || []));
  }, [eventId]);

  useEffect(() => {
    if (!playing || stepIndex >= replay.length - 1) return;

    const timer = setTimeout(() => setStepIndex(stepIndex + 1), 1000);
    return () => clearTimeout(timer);
  }, [stepIndex, playing, replay.length]);

  const handlePlay = () => {
    setStepIndex(0);
    setPlaying(true);
  };

  const handleStop = () => {
    setPlaying(false);
    setStepIndex(0);
  };

  return (
    <div className="mt-6 border-t pt-4">
      <h3 className="text-lg font-semibold text-gray-800 mb-2">📽️ GNN Attack Replay</h3>
      <div className="mb-2">
        <button onClick={handlePlay} className="mr-2 px-3 py-1 bg-blue-600 text-white rounded">▶️ Play</button>
        <button onClick={handleStop} className="px-3 py-1 bg-gray-500 text-white rounded">⏹ Stop</button>
      </div>

      {replay.slice(0, stepIndex + 1).map((step, i) => (
        <div key={i} className="p-3 bg-gray-100 rounded mb-2 text-sm">
          <div><strong>Step {i + 1}</strong> — {step.timestamp}</div>
          <div>🔗 Node: {step.node_id}</div>
          <div>🧠 SHAP Top Feature: {step.shap_top}</div>
          <div>📉 Trust Δ: {step.trust_delta}</div>
          <div>📦 Event Type: {step.event_type}</div>
        </div>
      ))}
    </div>
  );
};

export default GNNReplay;
