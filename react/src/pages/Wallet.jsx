import { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';

const Wallet = () => {
  const [isOnboarded, setIsOnboarded] = useState(false);
  const [isOnboarding, setIsOnboarding] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();
  const location = useLocation();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    // Check if wallet is already onboarded
    const onboarded = localStorage.getItem('walletOnboarded') === 'true';
    setIsOnboarded(onboarded);

    // If onboarded and on base wallet path, redirect to DID page
    if (onboarded && location.pathname === '/wallet') {
      navigate('/wallet/did');
    }
  }, [location.pathname, navigate]);

  const handleOnboard = async () => {
    setIsOnboarding(true);
    setError(null);

    try {
      const response = await fetch(`${apiUrl}/wallet/onboard`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to onboard wallet');
      }

      // Mark as onboarded in localStorage
      localStorage.setItem('walletOnboarded', 'true');
      setIsOnboarded(true);

      // Navigate to DID page
      navigate('/wallet/did');
    } catch (err) {
      console.error('Error onboarding wallet:', err);
      setError(err.message);
    } finally {
      setIsOnboarding(false);
    }
  };

  const isActiveTab = (path) => {
    return location.pathname === path;
  };

  const tabStyle = (path) => ({
    padding: '12px 24px',
    textDecoration: 'none',
    color: isActiveTab(path) ? '#00f0ff' : '#e0e0e0',
    backgroundColor: isActiveTab(path) ? 'rgba(0, 240, 255, 0.15)' : 'transparent',
    borderBottom: isActiveTab(path) ? '3px solid #00f0ff' : '3px solid transparent',
    transition: 'all 0.3s ease',
    cursor: 'pointer',
    fontWeight: isActiveTab(path) ? 'bold' : 'normal',
    textShadow: isActiveTab(path) ? '0 0 10px rgba(0, 240, 255, 0.8)' : 'none',
    border: 'none',
    fontSize: '1em',
    fontFamily: 'inherit',
  });

  if (!isOnboarded) {
    return (
      <div style={{ padding: '30px', width: '100%', minHeight: '100vh', textAlign: 'center' }}>
        <h1>Wallet</h1>
        <div
          style={{
            border: '2px solid #00f0ff',
            padding: '40px',
            borderRadius: '8px',
            marginTop: '40px',
            maxWidth: '500px',
            margin: '40px auto',
            backgroundColor: 'rgba(26, 29, 53, 0.6)',
            boxShadow: '0 0 20px rgba(0, 240, 255, 0.3)',
          }}
        >
          <p style={{ fontSize: '1.2em', marginBottom: '30px', color: '#e0e0e0' }}>
            Link your wallet to get started
          </p>
          <button
            onClick={handleOnboard}
            disabled={isOnboarding}
            style={{
              padding: '15px 40px',
              fontSize: '1.2em',
              fontWeight: 'bold',
              cursor: isOnboarding ? 'not-allowed' : 'pointer',
              backgroundColor: isOnboarding ? 'rgba(189, 0, 255, 0.1)' : 'rgba(189, 0, 255, 0.2)',
              border: '2px solid #bd00ff',
              color: '#bd00ff',
              borderRadius: '4px',
              boxShadow: '0 0 15px rgba(189, 0, 255, 0.4)',
              transition: 'all 0.3s ease',
            }}
            onMouseEnter={(e) => {
              if (!isOnboarding) {
                e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.3)';
                e.currentTarget.style.boxShadow = '0 0 20px rgba(189, 0, 255, 0.6)';
              }
            }}
            onMouseLeave={(e) => {
              if (!isOnboarding) {
                e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.2)';
                e.currentTarget.style.boxShadow = '0 0 15px rgba(189, 0, 255, 0.4)';
              }
            }}
          >
            {isOnboarding ? 'LINKING...' : 'LINK'}
          </button>
          {error && <p style={{ color: '#ff0040', marginTop: '20px' }}>Error: {error}</p>}
        </div>
      </div>
    );
  }

  return (
    <div style={{ padding: '30px', width: '100%', minHeight: '100vh' }}>
      <h1>Wallet</h1>

      {/* Sub-navigation tabs */}
      <div
        style={{
          display: 'flex',
          gap: '0',
          borderBottom: '2px solid #00f0ff',
          marginBottom: '30px',
          marginTop: '20px',
        }}
      >
        <button
          onClick={() => navigate('/wallet/did')}
          style={tabStyle('/wallet/did')}
          onMouseEnter={(e) => {
            if (!isActiveTab('/wallet/did')) {
              e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
            }
          }}
          onMouseLeave={(e) => {
            if (!isActiveTab('/wallet/did')) {
              e.currentTarget.style.backgroundColor = 'transparent';
            }
          }}
        >
          DID
        </button>
        <button
          onClick={() => navigate('/wallet/info')}
          style={tabStyle('/wallet/info')}
          onMouseEnter={(e) => {
            if (!isActiveTab('/wallet/info')) {
              e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
            }
          }}
          onMouseLeave={(e) => {
            if (!isActiveTab('/wallet/info')) {
              e.currentTarget.style.backgroundColor = 'transparent';
            }
          }}
        >
          Info
        </button>
        <button
          onClick={() => navigate('/wallet/credentials')}
          style={tabStyle('/wallet/credentials')}
          onMouseEnter={(e) => {
            if (!isActiveTab('/wallet/credentials')) {
              e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
            }
          }}
          onMouseLeave={(e) => {
            if (!isActiveTab('/wallet/credentials')) {
              e.currentTarget.style.backgroundColor = 'transparent';
            }
          }}
        >
          Credentials
        </button>
        <button
          onClick={() => navigate('/wallet/oidc4vp')}
          style={tabStyle('/wallet/oidc4vp')}
          onMouseEnter={(e) => {
            if (!isActiveTab('/wallet/oidc4vp')) {
              e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
            }
          }}
          onMouseLeave={(e) => {
            if (!isActiveTab('/wallet/oidc4vp')) {
              e.currentTarget.style.backgroundColor = 'transparent';
            }
          }}
        >
          OIDC4VP
        </button>
        <button
          onClick={() => navigate('/wallet/oidc4vci')}
          style={tabStyle('/wallet/oidc4vci')}
          onMouseEnter={(e) => {
            if (!isActiveTab('/wallet/oidc4vci')) {
              e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
            }
          }}
          onMouseLeave={(e) => {
            if (!isActiveTab('/wallet/oidc4vci')) {
              e.currentTarget.style.backgroundColor = 'transparent';
            }
          }}
        >
          OIDC4VCI
        </button>
      </div>

      {/* Sub-page content */}
      <Outlet />
    </div>
  );
};

export default Wallet;
