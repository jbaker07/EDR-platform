import React, { useEffect, useState } from 'react';
import axios from 'axios';

const TrustPanel: React.FC<{ hostname: string }> = ({ hostname }) => {
  const [score, setScore] = useState<number | null>(null);
  const [status, setStatus] = useState('Loading...');

  useEffect(() => {
    const fetchTrust = async () => {
      try {
        // Per-host trust route in backend/app/routes/trust_router.py.
        // The old /api/trust/{hostname} path never existed on the backend.
        const res = await axios.get(`/api/host/${encodeURIComponent(hostname)}`);
        setScore(res.data.trust_score);
      } catch (err) {
        setScore(null);
        const notFound = axios.isAxiosError(err) && err.response?.status === 404;
        setStatus(notFound ? 'No telemetry for this host' : 'Could not load trust score');
        console.error('Error fetching trust score:', err);
      }
    };

    fetchTrust();
    const interval = setInterval(fetchTrust, 10000);
    return () => clearInterval(interval);
  }, [hostname]);

  return (
    <div className="bg-white shadow-md rounded-xl p-4 mb-6">
      <h2 className="text-lg font-semibold mb-2">🔐 Trust Score</h2>
      <p className="text-2xl font-bold text-gray-900">{score !== null ? score : status}</p>
      <p className="text-sm text-gray-500 mt-1">Host: {hostname}</p>
    </div>
  );
};

export default TrustPanel;
