import { useState, useEffect } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { useNotifications } from '@/contexts/NotificationContext';

const Wallet = () => {
  const { isWalletLinked, isWalletLoading, checkWalletStatus } = useNotifications();
  const [isLinking, setIsLinking] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();
  const location = useLocation();

  const getButtonText = () => {
    if (isLinking) return 'LINKING...';
    if (isWalletLinked) return 'LINK AGAIN';
    if (error) return 'RETRY LINK';
    return 'LINK';
  };

  const handleOnboard = async () => {
    setIsLinking(true);
    setError(null);

    try {
      const response = await fetch(`${apiUrl}/wallet/link`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to link wallet');
      }

      // Refresh global status
      await checkWalletStatus();

      // Navigate to DID page if we were on the base wallet path
      if (location.pathname === '/wallet') {
        navigate('/wallet/info', { replace: true });
      }
    } catch (err) {
      console.error('Error onboarding wallet:', err);
      setError(err.message);
    } finally {
      setIsLinking(false);
    }
  };

  useEffect(() => {
    if (isWalletLoading) return;

    // If onboarded and on base wallet path, redirect to DID page
    if (isWalletLinked && location.pathname === '/wallet') {
      navigate('/wallet/info', { replace: true });
    } else if (!isWalletLinked && location.pathname === '/wallet') {
      // Check for auto-link request from Notifications
      const searchParams = new URLSearchParams(location.search);
      if (searchParams.get('autolink') === 'true') {
        const timer = setTimeout(() => {
          handleOnboard();
        }, 300); // small delay for UI rendering before firing network request
        return () => clearTimeout(timer);
      }
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [location.pathname, location.search, navigate, isWalletLinked, isWalletLoading]);


  const isActiveTab = (path) => {
    return location.pathname === path;
  };

  if (isWalletLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-brand-sky animate-pulse">Checking wallet status...</div>
      </div>
    );
  }

  return (
    <div className="w-full min-h-screen">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-3xl font-bold text-brand-sky font-ubuntu">Wallet</h1>

        {/* Always visible Link Button */}
        <Button
          onClick={handleOnboard}
          disabled={isLinking}
          variant={error ? 'destructive' : 'default'}
          className={cn(
            'font-bold transition-all shadow-lg',
            isWalletLinked ? 'bg-success hover:bg-success/90 text-white' : '',
            !isWalletLinked && !error
              ? 'bg-brand-purple hover:bg-brand-purple/90 text-white shadow-brand-purple/40'
              : '',
          )}
        >
          {getButtonText()}
        </Button>
      </div>

      {error && !isWalletLinked && (
        <div className="mb-6 p-4 rounded-md border border-danger bg-danger/10 text-danger">
          Error: {error}
        </div>
      )}

      {/* Sub-navigation tabs - Only visible if onboarded */}
      {isWalletLinked && (
        <>
          <div className="flex border-b border-brand-sky mb-8 mt-6">
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
