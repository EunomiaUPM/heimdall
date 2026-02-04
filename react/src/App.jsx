import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import Home from './pages/Home';
import About from './pages/About';
import Dashboard from './pages/Dashboard';
import Minions from './pages/Minions';
import MinionDetails from './pages/MinionDetails';
import Requests from './pages/Requests';
import RequestDetails from './pages/RequestDetails';
import './App.css';

function App() {
  // Accessing environment variables
  const featureFlag1 = import.meta.env.VITE_FEATURE_FLAG_1 === 'true';
  const featureFlag2 = import.meta.env.VITE_FEATURE_FLAG_2 === 'true';

  console.log('Feature Flag 1:', featureFlag1);
  console.log('Feature Flag 2:', featureFlag2);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="about" element={<About />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="minions" element={<Minions />} />
          <Route path="minions/:id" element={<MinionDetails />} />
          <Route path="requests" element={<Requests />} />
          <Route path="requests/:id" element={<RequestDetails />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
