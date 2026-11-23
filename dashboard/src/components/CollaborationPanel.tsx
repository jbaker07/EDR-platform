// dashboard/src/components/CollaborationPanel.tsx
import { useCallback, useEffect, useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { apiJson, postJson } from '@/api/client';

// Same fields as the Message model in backend/app/routes/collaboration.py.
// The server sets timestamp.
interface Message {
  author: string;
  content: string;
  timestamp?: string;
}

export default function CollaborationPanel() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [author, setAuthor] = useState('');
  const [content, setContent] = useState('');
  const [error, setError] = useState<string | null>(null);

  const fetchMessages = useCallback(() => {
    apiJson<Message[]>('/api/collaboration')
      .then((data) => { setMessages(data); setError(null); })
      .catch((err) => setError(err.message));
  }, []);

  const postMessage = async () => {
    if (!author.trim() || !content.trim()) return;
    try {
      await postJson('/api/collaboration', { author: author.trim(), content: content.trim() });
      setContent('');
      fetchMessages();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  useEffect(() => {
    fetchMessages();
  }, [fetchMessages]);

  return (
    <div className="bg-white p-6 rounded-2xl shadow-xl mb-10">
      <h2 className="text-xl font-semibold mb-4">Analyst Notes</h2>
      {error && <p className="text-sm text-red-600 mb-2">{error}</p>}
      <div className="space-y-3 max-h-64 overflow-y-auto border rounded p-3 mb-4">
        {messages.length === 0 && <p className="text-sm text-gray-500">No notes yet.</p>}
        {messages.map((m, i) => (
          <div key={i} className="border-b pb-2">
            <p className="text-sm text-gray-800">{m.content}</p>
            <p className="text-xs text-gray-500">
              {m.author}
              {m.timestamp && <> • {new Date(m.timestamp).toLocaleString()}</>}
            </p>
          </div>
        ))}
      </div>
      <div className="flex gap-2">
        <Input className="w-40" placeholder="Your name" value={author} onChange={(e) => setAuthor(e.target.value)} />
        <Input placeholder="Add a note..." value={content} onChange={(e) => setContent(e.target.value)} />
        <Button onClick={postMessage}>Send</Button>
      </div>
    </div>
  );
}
