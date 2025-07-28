// src/main.tsx

// ✅ Import the Chart.js date-fns adapter before any ChartJS.register calls
import 'chartjs-adapter-date-fns';

import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import App from './App.tsx';
import 'react-toastify/dist/ReactToastify.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
