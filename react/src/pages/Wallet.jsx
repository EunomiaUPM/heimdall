import { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation, Link } from 'react-router-dom';
import { Wallet as WalletIcon, Loader2 } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { useNotifications } from '@/contexts/NotificationContext';

const tabs = [
  { label: 'Info', to: '/wallet/info' },
  { label: 'DID', to: '/wallet/did' },
  { label: 'Credentials', to: '/wallet/credentials' },
  { label: 'OIDC4VP', to: '/wallet/oidc4vp' },
  { label: 'OIDC4VCI', to: '/wallet/oidc4vci' },
];

const Wallet = () => {
  const { isWalletLinked, isWalletLoading, checkWalletStatus } = useNotifications();
  const [isLinking, setIsLinking] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();
  const location = useLocation();

  const handleOnboard = async () => {
    setIsLinking(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/wallet/link`, { method: 'POST' });
      if (!response.ok) throw new Error('Failed to link wallet');
      await checkWalletStatus();
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
    if (isWalletLinked && location.pathname === '/wallet') {
      navigate('/wallet/info', { replace: true });
    } else if (!isWalletLinked && location.pathname === '/wallet') {
      const searchParams = new URLSearchParams(location.search);
      if (searchParams.get('autolink') === 'true') {
        const timer = setTimeout(handleOnboard, 300);
        return () => clearTimeout(timer);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [location.pathname, location.search, navigate, isWalletLinked, isWalletLoading]);

  if (isWalletLoading) {
    return (
      <PageLayout>
        <PageHeader title="Wallet" />
        <p className="text-brand-sky animate-pulse italic text-sm">Checking wallet status…</p>
      </PageLayout>
    );
  }

  return (
    <PageLayout>
      <PageHeader title="Wallet">
        <div className="flex items-center gap-3">
          <Button
            onClick={handleOnboard}
            disabled={isLinking}
            variant={isWalletLinked ? 'outline' : 'default'}
            size="sm"
            className="flex gap-2"
          >
            {isLinking ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <WalletIcon className="h-4 w-4" />
            )}
            {isLinking ? 'Linking…' : isWalletLinked ? 'Re-Link Wallet' : 'Link Wallet'}
          </Button>
        </div>
      </PageHeader>

      {error && !isWalletLinked && (
        <div className="mb-6 p-3 rounded-md border border-destructive/30 bg-destructive/10 text-sm text-destructive flex gap-2 items-start">
          <span className="font-semibold">Error:</span>
          <span>{error}</span>
        </div>
      )}

      {!isWalletLinked ? (
        <div className="flex flex-col items-center justify-center min-h-[400px] border border-dashed border-white/10 rounded-xl bg-white/5 p-12">
          <WalletIcon className="h-12 w-12 text-muted-foreground/40 mb-4" />
          <p className="text-muted-foreground font-medium text-lg">Wallet not linked</p>
          <p className="text-muted-foreground/60 text-sm mt-1 mb-6">
            Link your wallet to start managing your identity.
          </p>
          <Button onClick={handleOnboard} disabled={isLinking}>
            {isLinking ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" /> Linking…
              </>
            ) : (
              'Link Wallet Now'
            )}
          </Button>
        </div>
      ) : (
        <>
          <div className="flex gap-1 border-b border-white/10 mb-6 w-full">
            {tabs.map((tab) => {
              const isActive =
                location.pathname === tab.to || location.pathname.startsWith(tab.to + '/');
              return (
                <Link
                  key={tab.to}
                  to={tab.to}
                  className={cn(
                    'px-4 py-2 text-sm font-medium border-b-2 transition-all',
                    isActive
                      ? 'border-primary text-primary'
                      : 'border-transparent text-muted-foreground hover:text-foreground',
                  )}
                >
                  {tab.label}
                </Link>
              );
            })}
          </div>
          <Outlet />
        </>
      )}
    </PageLayout>
  );
};

export default Wallet;
