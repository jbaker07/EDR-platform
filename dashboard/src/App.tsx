import { useEffect, useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { ShieldAlert, ActivitySquare, ListChecks, Search, MessageSquare, BarChart3, Siren } from 'lucide-react';

import { setToken, setUnauthorizedHandler } from '@/api/client';
import TokenGate from '@/components/TokenGate';
import Dashboard from './pages/Dashboard';
import Processes from './pages/Processes';
import FeedbackPage from './pages/FeedbackPage';
import AgentIncidents from './pages/AgentIncidents';
import DetectionLogicPanel from '@/components/DetectionLogicPanel';
import IOCViewerPanel from '@/components/IOCViewerPanel';
import CollaborationPanel from '@/components/CollaborationPanel';

export default function App() {
  const [signedIn, setSignedIn] = useState(false);
  const [gateMessage, setGateMessage] = useState<string | null>(null);

  useEffect(() => {
    setUnauthorizedHandler(() => {
      setGateMessage('The backend rejected the token. Sign in again.');
      setSignedIn(false);
    });
    return () => setUnauthorizedHandler(null);
  }, []);

  if (!signedIn) {
    return <TokenGate message={gateMessage} onSignedIn={() => { setGateMessage(null); setSignedIn(true); }} />;
  }

  const signOut = () => {
    setToken(null);
    setSignedIn(false);
  };

  return (
    <Router>
      <div className="min-h-screen flex bg-gray-100 text-gray-900">
        <aside className="w-64 bg-white shadow-md p-6 block">
          <h2 className="text-2xl font-bold mb-6">EDR Console</h2>
          <nav className="space-y-4 text-sm font-medium">
            <Link to="/" className="flex items-center gap-2 hover:text-blue-600">
              <ShieldAlert size={18} /> <span>Dashboard</span>
            </Link>
            <Link to="/incidents" className="flex items-center gap-2 hover:text-blue-600">
              <Siren size={18} /> <span>Agent Incidents</span>
            </Link>
            <Link to="/processes" className="flex items-center gap-2 hover:text-blue-600">
              <ActivitySquare size={18} /> <span>Processes</span>
            </Link>
            <Link to="/detection" className="flex items-center gap-2 hover:text-blue-600">
              <ListChecks size={18} /> <span>Detection Rules</span>
            </Link>
            <Link to="/iocs" className="flex items-center gap-2 hover:text-blue-600">
              <Search size={18} /> <span>IOCs</span>
            </Link>
            <Link to="/feedback" className="flex items-center gap-2 hover:text-blue-600">
              <BarChart3 size={18} /> <span>Feedback Log</span>
            </Link>
            <Link to="/collab" className="flex items-center gap-2 hover:text-blue-600">
              <MessageSquare size={18} /> <span>Collaboration</span>
            </Link>
          </nav>
          <button onClick={signOut} className="mt-8 text-sm text-gray-500 hover:text-red-600">
            Sign out
          </button>
        </aside>

        <main className="flex-1 p-6 max-w-7xl mx-auto">
          <header className="sticky top-0 bg-gray-100 z-10 py-4 mb-6 border-b border-gray-200">
            <h1 className="text-3xl font-semibold">Endpoint Detection Dashboard</h1>
          </header>

          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/incidents" element={<AgentIncidents />} />
            <Route path="/processes" element={<Processes />} />
            <Route path="/detection" element={<DetectionLogicPanel />} />
            <Route path="/iocs" element={<IOCViewerPanel />} />
            <Route path="/feedback" element={<FeedbackPage />} />
            <Route path="/collab" element={<CollaborationPanel />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}
