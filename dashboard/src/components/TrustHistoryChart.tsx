import React, { useEffect, useState } from 'react';
import {
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer
} from 'recharts';

interface TrustPoint {
  timestamp: string;
  trust_score: number;
}

interface Props {
  hostname: string;
}

const TrustHistoryChart: React.FC<Props> = ({ hostname }) => {
  const [data, setData] = useState<TrustPoint[]>([]);

  useEffect(() => {
    fetch(`/api/trust/history?hostname=${hostname}`)
      .then(res => res.json())
      .then(setData)
      .catch(() => setData([]));
  }, [hostname]);

  return (
    <div className="mt-6 border-t pt-4">
      <h3 className="font-semibold text-gray-800 mb-2">Trust Score Over Time</h3>
      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="timestamp" tickFormatter={(t) => t.split('T')[1].slice(0, 5)} />
          <YAxis domain={[0, 100]} />
          <Tooltip />
          <Line type="monotone" dataKey="trust_score" stroke="#8884d8" strokeWidth={2} />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

export default TrustHistoryChart;
