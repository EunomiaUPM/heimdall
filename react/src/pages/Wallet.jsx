import { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';

const Wallet = () => {
  const [isOnboarded, setIsOnboarded] = useState(false);
  const [isOnboarding, setIsOnboarding] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();
  const location = useLocation();

  // Determine button state color
  // Default (not onboarded, not error): Purple
  // Error: Red
  // Onboarded: Green
  const getButtonColor = () => {
    if (isOnboarded) return '#00ff00'; // Green
    if (error) return '#ff0000'; // Red
    return '#bd00ff'; // Purple
  };

  const getButtonText = () => {
    if (isOnboarding) return 'LINKING...';
    if (isOnboarded) return 'LINKED';
    if (error) return 'RETRY LINK';
    return 'LINK';
  };

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
    if (isOnboarded) return; // Do nothing if already linked

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

  const buttonColor = getButtonColor();

  return (
    <div style={{ padding: '30px', width: '100%', minHeight: '100vh' }}>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: '30px',
        }}
      >
        <h1>Wallet</h1>

        {/* Always visible Link Button */}
        <button
          onClick={handleOnboard}
          disabled={isOnboarding || isOnboarded}
          style={{
            padding: '10px 30px',
            fontSize: '1em',
            fontWeight: 'bold',
            cursor: isOnboarding || isOnboarded ? 'default' : 'pointer',
            backgroundColor: isOnboarding
              ? `rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.1)`
              : `rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.2)`,
            border: `2px solid ${buttonColor}`,
            color: buttonColor,
            borderRadius: '4px',
            boxShadow: `0 0 15px rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.4)`,
            transition: 'all 0.3s ease',
          }}
          onMouseEnter={(e) => {
            if (!isOnboarding && !isOnboarded) {
              e.currentTarget.style.backgroundColor = `rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.3)`;
              e.currentTarget.style.boxShadow = `0 0 20px rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.6)`;
            }
          }}
          onMouseLeave={(e) => {
            if (!isOnboarding && !isOnboarded) {
              e.currentTarget.style.backgroundColor = `rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.2)`;
              e.currentTarget.style.boxShadow = `0 0 15px rgba(${parseInt(buttonColor.slice(1, 3), 16)}, ${parseInt(buttonColor.slice(3, 5), 16)}, ${parseInt(buttonColor.slice(5, 7), 16)}, 0.4)`;
            }
          }}
        >
          {getButtonText()}
        </button>
      </div>

      {error && !isOnboarded && (
        <div
          style={{
            color: '#ff0040',
            marginBottom: '20px',
            padding: '10px',
            border: '1px solid #ff0040',
            borderRadius: '4px',
            backgroundColor: 'rgba(255, 0, 64, 0.1)',
          }}
        >
          Error: {error}
        </div>
      )}

      {/* Sub-navigation tabs - Only visible if onboarded */}
      {isOnboarded && (
        <>
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
        </>
      )}
    </div>
  );
};

export default Wallet;
