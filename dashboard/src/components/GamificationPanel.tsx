import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Medal } from 'lucide-react';

interface Analyst {
  username: string;
  points: number;
  badges: string[];
  last_active: string;
}

export default function GamificationPanel() {
  const [analysts, setAnalysts] = useState<Analyst[]>([]);

  useEffect(() => {
    axios.get('/api/gamification')
      .then(res => setAnalysts(res.data))
      .catch(err => console.error('Failed to fetch gamification data', err));
  }, []);

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">🏆 SOC Gamification Leaderboard</h2>
      <table className="min-w-full text-sm text-left border">
        <thead className="bg-gray-100 text-gray-600">
          <tr>
            <th className="p-2">👤 Analyst</th>
            <th className="p-2">🎯 Points</th>
            <th className="p-2">🏅 Badges</th>
            <th className="p-2">🕒 Last Active</th>
          </tr>
        </thead>
        <tbody>
          {analysts.map((a, i) => (
            <tr key={i} className="border-t hover:bg-gray-50">
              <td className="p-2 font-medium">{a.username}</td>
              <td className="p-2">{a.points}</td>
              <td className="p-2 space-x-1">
                {a.badges.map((b, idx) => (
                  <span key={idx} className="inline-block px-2 py-1 bg-blue-100 text-blue-700 text-xs rounded">
                    {b}
                  </span>
                ))}
              </td>
              <td className="p-2 text-gray-500">{new Date(a.last_active).toLocaleString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
