import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import Home from './pages/Home';
import About from './pages/About';
import Minions from './pages/Minions';
import MinionDetails from './pages/MinionDetails';
import Requests from './pages/Requests';
import RequestDetails from './pages/RequestDetails';
import Wallet from './pages/Wallet';
import WalletDID from './pages/WalletDID';
import WalletInfo from './pages/WalletInfo';
import WalletCredentials from './pages/WalletCredentials';
import './App.css';

function App() {
  // Accessing environment variables
  const featureFlag1 = import.meta.env.VITE_FEATURE_FLAG_1 === 'true';
  const featureFlag2 = import.meta.env.VITE_FEATURE_FLAG_2 === 'true';
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE === 'true';

  console.log('Feature Flag 1:', featureFlag1);
  console.log('Feature Flag 2:', featureFlag2);
  console.log('Wallet Active:', walletActive);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="about" element={<About />} />
          <Route path="minions" element={<Minions />} />
          <Route path="minions/:id" element={<MinionDetails />} />
          <Route path="requests" element={<Requests />} />
          <Route path="requests/:id" element={<RequestDetails />} />
          {walletActive && (
            <Route path="wallet" element={<Wallet />}>
              <Route path="did" element={<WalletDID />} />
              <Route path="info" element={<WalletInfo />} />
              <Route path="credentials" element={<WalletCredentials />} />
            </Route>
          )}
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
