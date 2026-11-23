// src/components/TokenGate.tsx
import React, { useState } from 'react';
import { apiFetch, setToken } from '@/api/client';

interface TokenGateProps {
  message: string | null;
  onSignedIn: () => void;
}

// Asks for the API token and checks it against /api/health before letting the
// rest of the dashboard load.
export default function TokenGate({ message, onSignedIn }: TokenGateProps) {
  const [value, setValue] = useState('');
  const [error, setError] = useState<string | null>(message);
  const [checking, setChecking] = useState(false);

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    setChecking(true);
    setToken(value.trim());
    try {
      const res = await apiFetch('/api/health');
      if (res.ok) {
        setValue('');
        onSignedIn();
        return;
      }
      setError(res.status === 401 ? 'That token was rejected.' : `Backend answered HTTP ${res.status}.`);
    } catch {
      setError('Could not reach the backend.');
    } finally {
      setChecking(false);
    }
    setToken(null);
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100 text-gray-900">
      <form onSubmit={submit} className="bg-white p-6 rounded-xl shadow-md w-96 space-y-4">
        <h1 className="text-xl font-semibold">EDR Console</h1>
        <label className="block text-sm">
          API token
          <input
            type="password"
            autoComplete="off"
            className="mt-1 w-full border rounded p-2"
            value={value}
            onChange={(e) => setValue(e.target.value)}
          />
        </label>
        <p className="text-xs text-gray-500">
          The value of EDR_API_TOKEN on the backend. It is kept in memory only, so a reload asks again.
        </p>
        {error && <p className="text-sm text-red-600">{error}</p>}
        <button
          type="submit"
          disabled={checking || value.trim() === ''}
          className="w-full bg-blue-600 text-white rounded py-2 disabled:opacity-50"
        >
          {checking ? 'Checking...' : 'Sign in'}
        </button>
      </form>
    </div>
  );
}
