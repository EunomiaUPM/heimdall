import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
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
import WalletOidc4vp from './pages/WalletOidc4vp';
import WalletOidc4vci from './pages/WalletOidc4vci';

function App() {
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE === 'true';

  return (
    <BrowserRouter basename={import.meta.env.BASE_URL}>
      <Routes>
        <Route path="/" element={<Layout />}>
          {/* Redirect root to /home */}
          <Route index element={<Navigate to="/home" replace />} />
          <Route path="home" element={<Home />} />
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
              <Route path="oidc4vp" element={<WalletOidc4vp />} />
              <Route path="oidc4vci" element={<WalletOidc4vci />} />
            </Route>
          )}
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
