// dashboard/src/components/CollaborationPanel.tsx
import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

interface Comment {
  author: string;
  message: string;
  timestamp: string;
}

export default function CollaborationPanel() {
  const [comments, setComments] = useState<Comment[]>([]);
  const [message, setMessage] = useState('');

  const fetchComments = async () => {
    const res = await axios.get('/api/collaboration');
    setComments(res.data);
  };

  const postComment = async () => {
    if (!message.trim()) return;
    await axios.post('/api/collaboration', {
      author: 'Analyst A',
      message,
      timestamp: new Date().toISOString(),
    });
    setMessage('');
    fetchComments();
  };

  useEffect(() => {
    fetchComments();
  }, []);

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">💬 Cross-SOC Collaboration</h2>
      <div className="space-y-3 max-h-64 overflow-y-auto border rounded p-3 mb-4">
        {comments.map((c, i) => (
          <div key={i} className="border-b pb-2">
            <p className="text-sm text-gray-800">{c.message}</p>
            <p className="text-xs text-gray-500">{c.author} • {new Date(c.timestamp).toLocaleString()}</p>
          </div>
        ))}
      </div>
      <div className="flex gap-2">
        <Input
          placeholder="Add a comment..."
          value={message}
          onChange={(e) => setMessage(e.target.value)}
        />
        <Button onClick={postComment}>Send</Button>
      </div>
    </div>
  );
}