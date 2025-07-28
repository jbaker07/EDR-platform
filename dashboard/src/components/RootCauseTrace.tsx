import React, { useEffect, useState } from 'react';

interface TraceData {
  origin: string[];
  compromised: string;
  neighbors: string[];
}

interface Props {
  eventId: string;
}

const RootCauseTrace: React.FC<Props> = ({ eventId }) => {
  const [trace, setTrace] = useState<TraceData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`/api/explain/root_cause?event_id=${eventId}`)
      .then(res => res.json())
      .then(data => {
        setTrace(data);
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, [eventId]);

  if (loading) return <p className="text-sm text-gray-500">Loading root cause trace...</p>;
  if (!trace) return <p className="text-sm text-red-500">Failed to load trace.</p>;

  return (
    <div className="mt-6 border-t pt-4">
      <h3 className="font-semibold text-gray-800 mb-1">Root Cause Trace</h3>
      <p className="text-sm text-gray-700">
        <strong>Origin Nodes:</strong> {trace.origin.join(', ') || 'Unknown'}
      </p>
      <p className="text-sm text-gray-700">
        <strong>Compromised Node:</strong> {trace.compromised}
      </p>
      <p className="text-sm text-gray-700">
        <strong>Connected Nodes:</strong> {trace.neighbors.join(', ') || 'None'}
      </p>
    </div>
  );
};

export default RootCauseTrace;
