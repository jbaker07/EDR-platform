import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import NotificationBanner from './components/NotificationBanner';
import { ShieldAlert, Server, ActivitySquare, BarChart3, BookOpenCheck, Sparkles } from 'lucide-react';

// Pages
import Dashboard from './pages/Dashboard';
import Processes from './pages/Processes';
import PlaybooksIOCs from './pages/PlaybooksIOCs';
import Forensics from './pages/Forensics';
import CollaborationThreat from './pages/CollaborationThreat';
import DetectionLogicPage from './pages/DetectionLogicPage';
import IsolationPage from './pages/IsolationPage';
import ExplainPage from './pages/ExplainPage';
import IntelPage from './pages/IntelPage';
import FeedbackPage from './pages/FeedbackPage';
import GamificationPage from './pages/GamificationPage'; // ✅ New

export default function App() {
  return (
    <Router>
      <div className="min-h-screen flex bg-gray-100 text-gray-900">
        {/* Sidebar */}
        <aside className="w-64 bg-white shadow-md p-6 block">
          <h2 className="text-2xl font-bold mb-6">EDR Console</h2>
          <nav className="space-y-4 text-sm font-medium">
            <Link to="/" className="flex items-center gap-2 hover:text-blue-600">
              <ShieldAlert size={18} /> <span>Dashboard</span>
            </Link>
            <Link to="/processes" className="flex items-center gap-2 hover:text-blue-600">
              <ActivitySquare size={18} /> <span>Processes</span>
            </Link>
            <Link to="/playbooks" className="flex items-center gap-2 hover:text-blue-600">
              <BookOpenCheck size={18} /> <span>Playbooks</span>
            </Link>
            <Link to="/forensics" className="flex items-center gap-2 hover:text-blue-600">
              <Server size={18} /> <span>Forensics</span>
            </Link>
            <Link to="/collab" className="flex items-center gap-2 hover:text-blue-600">
              <BarChart3 size={18} /> <span>Collaboration</span>
            </Link>
            <Link to="/detection" className="flex items-center gap-2 hover:text-blue-600">
              <ActivitySquare size={18} /> <span>Detection Logic</span>
            </Link>
            <Link to="/isolation" className="flex items-center gap-2 hover:text-blue-600">
              <ShieldAlert size={18} /> <span>Isolation</span>
            </Link>
            <Link to="/explain" className="flex items-center gap-2 hover:text-blue-600">
              <BarChart3 size={18} /> <span>Explainability</span>
            </Link>
            <Link to="/intel" className="flex items-center gap-2 hover:text-blue-600">
              <Server size={18} /> <span>Threat Intel</span>
            </Link>
            <Link to="/feedback" className="flex items-center gap-2 hover:text-blue-600">
              <BarChart3 size={18} /> <span>Feedback Log</span>
            </Link>
            <Link to="/gamification" className="flex items-center gap-2 hover:text-blue-600">
              <Sparkles size={18} /> <span>Gamification</span>
            </Link>
          </nav>
        </aside>

        {/* Main Content */}
        <main className="flex-1 p-6 max-w-7xl mx-auto">
          <NotificationBanner />

          <header className="sticky top-0 bg-gray-100 z-10 py-4 mb-6 border-b border-gray-200">
            <h1 className="text-3xl font-semibold">🔐 Endpoint Detection Dashboard</h1>
          </header>

          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/processes" element={<Processes />} />
            <Route path="/playbooks" element={<PlaybooksIOCs />} />
            <Route path="/forensics" element={<Forensics />} />
            <Route path="/collab" element={<CollaborationThreat />} />
            <Route path="/detection" element={<DetectionLogicPage />} />
            <Route path="/isolation" element={<IsolationPage />} />
            <Route path="/explain" element={<ExplainPage />} />
            <Route path="/intel" element={<IntelPage />} />
            <Route path="/feedback" element={<FeedbackPage />} />
            <Route path="/gamification" element={<GamificationPage />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}
