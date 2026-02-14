import { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

const Wallet = () => {
  const [isOnboarded, setIsOnboarded] = useState(false);
  const [isOnboarding, setIsOnboarding] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();
  const location = useLocation();

  // Determine button state color
  // Default (not onboarded, not error): Purple (brand-accent)
  // Error: Red (danger)
  // Onboarded: Green (success)
  // We'll use variants for this now
  const getButtonVariant = () => {
    if (isOnboarded) return 'default'; // Map to success style via className
    if (error) return 'destructive';
    return 'default'; // Map to brand style via className
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

  return (
    <div className="w-full min-h-screen">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-3xl font-bold text-brand-sky font-ubuntu">Wallet</h1>

        {/* Always visible Link Button */}
        <Button
          onClick={handleOnboard}
          disabled={isOnboarding || isOnboarded}
          variant={error ? 'destructive' : 'default'}
          className={cn(
            'font-bold transition-all shadow-lg',
            isOnboarded ? 'bg-success hover:bg-success/90 text-white' : '',
            !isOnboarded && !error
              ? 'bg-brand-purple hover:bg-brand-purple/90 text-white shadow-brand-purple/40'
              : '',
          )}
        >
          {getButtonText()}
        </Button>
      </div>

      {error && !isOnboarded && (
        <div className="mb-6 p-4 rounded-md border border-danger bg-danger/10 text-danger">
          Error: {error}
        </div>
      )}

      {/* Sub-navigation tabs - Only visible if onboarded */}
      {isOnboarded && (
        <>
          <div className="flex border-b border-brand-sky mb-8 mt-6">
            <button
              onClick={() => navigate('/wallet/did')}
              className={cn(
                'px-6 py-3 transition-colors font-medium border-b-2 -mb-[2px]',
                isActiveTab('/wallet/did')
                  ? 'border-brand-sky text-brand-sky bg-brand-sky/10'
                  : 'border-transparent text-gray-400 hover:text-brand-sky hover:bg-brand-sky/5',
              )}
            >
              DID
            </button>
            <button
              onClick={() => navigate('/wallet/info')}
              className={cn(
                'px-6 py-3 transition-colors font-medium border-b-2 -mb-[2px]',
                isActiveTab('/wallet/info')
                  ? 'border-brand-sky text-brand-sky bg-brand-sky/10'
                  : 'border-transparent text-gray-400 hover:text-brand-sky hover:bg-brand-sky/5',
              )}
            >
              Info
            </button>
            <button
              onClick={() => navigate('/wallet/credentials')}
              className={cn(
                'px-6 py-3 transition-colors font-medium border-b-2 -mb-[2px]',
                isActiveTab('/wallet/credentials')
                  ? 'border-brand-sky text-brand-sky bg-brand-sky/10'
                  : 'border-transparent text-gray-400 hover:text-brand-sky hover:bg-brand-sky/5',
              )}
            >
              Credentials
            </button>
            <button
              onClick={() => navigate('/wallet/oidc4vp')}
              className={cn(
                'px-6 py-3 transition-colors font-medium border-b-2 -mb-[2px]',
                isActiveTab('/wallet/oidc4vp')
                  ? 'border-brand-sky text-brand-sky bg-brand-sky/10'
                  : 'border-transparent text-gray-400 hover:text-brand-sky hover:bg-brand-sky/5',
              )}
            >
              OIDC4VP
            </button>
            <button
              onClick={() => navigate('/wallet/oidc4vci')}
              className={cn(
                'px-6 py-3 transition-colors font-medium border-b-2 -mb-[2px]',
                isActiveTab('/wallet/oidc4vci')
                  ? 'border-brand-sky text-brand-sky bg-brand-sky/10'
                  : 'border-transparent text-gray-400 hover:text-brand-sky hover:bg-brand-sky/5',
              )}
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
