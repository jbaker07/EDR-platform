import React, { useEffect, useState } from 'react';
import axios from 'axios';

const TrustPanel: React.FC<{ hostname: string }> = ({ hostname }) => {
  const [score, setScore] = useState<number | null>(null);

  useEffect(() => {
    const fetchTrust = async () => {
      try {
        const res = await axios.get(`http://localhost:8000/api/trust/${hostname}`);
        setScore(res.data.trust_score);
      } catch (err) {
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
      <p className="text-2xl font-bold text-gray-900">{score !== null ? score : 'Loading...'}</p>
      <p className="text-sm text-gray-500 mt-1">Host: {hostname}</p>
    </div>
  );
};

export default TrustPanel;
