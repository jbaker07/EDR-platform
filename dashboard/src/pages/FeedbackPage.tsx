import { useEffect, useState } from 'react';
import { apiJson } from '@/api/client';

interface FeedbackEntry {
  timestamp: string;
  source: 'analyst' | 'automation';
  type: 'false_positive' | 'false_negative' | 'suggestion';
  message: string;
  related_pid?: number;
  hostname?: string;
}

export default function FeedbackPage() {
  const [feedback, setFeedback] = useState<FeedbackEntry[]>([]);

  useEffect(() => {
    apiJson<FeedbackEntry[]>('/api/feedback')
      .then(setFeedback)
      .catch(() => setFeedback([]));
  }, []);

  return (
    <div className="bg-white p-6 rounded-xl shadow-xl">
      <h2 className="text-2xl font-semibold mb-4">Feedback Log</h2>
      {feedback.length === 0 ? (
        <p className="text-gray-500 text-sm">No feedback entries logged yet.</p>
      ) : (
        <ul className="divide-y divide-gray-200 text-sm">
          {feedback.map((entry, i) => (
            <li key={i} className="py-2">
              <p className="text-gray-800">
                <span className="font-medium capitalize">{entry.source}</span> - <span className="capitalize">{entry.type}</span>
              </p>
              <p className="text-gray-700">{entry.message}</p>
              <p className="text-gray-400 text-xs">
                {new Date(entry.timestamp).toLocaleString()}
                {entry.related_pid && <> • PID: {entry.related_pid}</>}
                {entry.hostname && <> • Host: {entry.hostname}</>}
              </p>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
