import { useEffect, useState, useCallback } from 'react';
import { Outlet, useNavigate, useLocation, Link } from 'react-router-dom';
import { Wallet as WalletIcon, Loader2 } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';

const tabs = [
  { label: 'Info', to: '/wallet/info' },
  { label: 'DID', to: '/wallet/did' },
  { label: 'Keys', to: '/wallet/keys' },
  { label: 'Credentials', to: '/wallet/credentials' },
  { label: 'OID4VP', to: '/wallet/oidc4vp' },
  { label: 'OID4VCI', to: '/wallet/oidc4vci' },
];

const Wallet = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [isWalletLinked, setIsWalletLinked] = useState(false);
  const [isLinking, setIsLinking] = useState(false);
  const [error, setError] = useState(null);

  const checkLinked = useCallback(async () => {
    try {
      const res = await fetch(`${apiUrl}/wallet/is-linked`);
      if (res.ok) {
        const data = await res.json();
        setIsWalletLinked(!!data?.is_linked);
      }
    } catch (err) {
      console.error('Error checking wallet status:', err);
    }
  }, []);

  useEffect(() => {
    checkLinked();
  }, [checkLinked]);

  useEffect(() => {
    if (location.pathname === '/wallet') {
      navigate('/wallet/info', { replace: true });
    }
  }, [location.pathname, navigate]);

  const handleLink = async () => {
    setIsLinking(true);
    setError(null);
    try {
      const res = await fetch(`${apiUrl}/wallet/link`, { method: 'POST' });
      if (!res.ok) throw new Error('Failed to link wallet');
      await checkLinked();
    } catch (err) {
      console.error('Error linking wallet:', err);
      setError(err.message);
    } finally {
      setIsLinking(false);
    }
  };

  return (
    <PageLayout>
      <PageHeader title="Wallet">
        <div className="flex items-center gap-3">
          <Button
            onClick={handleLink}
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

      {error && (
        <div className="mb-6 p-3 rounded-md border border-destructive/30 bg-destructive/10 text-sm text-destructive flex gap-2 items-start">
          <span className="font-semibold">Error:</span>
          <span>{error}</span>
        </div>
      )}

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
    </PageLayout>
  );
};

export default Wallet;
