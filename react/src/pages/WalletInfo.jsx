import { useState, useEffect } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { PageSection } from '@/components/layout/PageSection';
import { InfoList } from '@/components/ui/info-list';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';

const WalletInfo = () => {
  const [walletInfo, setWalletInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchWalletInfo = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/wallet/info`);
      if (!response.ok) throw new Error('Failed to fetch wallet info');
      const data = await response.json();
      setWalletInfo(data);
    } catch (err) {
      console.error('Error fetching wallet info:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchWalletInfo();
  }, []);

  if (loading) {
    return (
      <div className="space-y-8">
        <Skeleton className="h-40 w-full" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchWalletInfo} />;

  return (
    <div className="space-y-12 pb-20">
      <PageSection title="General Information">
        <div className="bg-white/5 border border-white/10 rounded-xl p-8 backdrop-blur-sm shadow-xl">
          <InfoList
            items={[
              { label: 'Wallet ID', value: walletInfo.id },
              { label: 'Name', value: walletInfo.name },
              { label: 'Created On', value: walletInfo.createdOn || '—' },
              { label: 'Permission Level', value: walletInfo.permission },
            ]}
          />
        </div>
      </PageSection>

      <PageSection title="Associated DIDs">
        <p className="text-xs text-muted-foreground mb-4">
          Quick overview of stored DIDs. Manage keys and defaults from the{' '}
          <span className="font-semibold text-primary">DID</span> tab.
        </p>
        <div className="rounded-md border border-white/10 bg-background-200/5">
          <Table className="text-sm">
            <TableHeader>
              <TableRow className="border-b-white/10 hover:bg-transparent">
                <TableHead className="text-white/80">Alias</TableHead>
                <TableHead className="text-white/80">DID</TableHead>
                <TableHead className="text-white/80">Type</TableHead>
                <TableHead className="text-white/80">Default</TableHead>
                <TableHead className="text-white/80"># Keys</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {walletInfo.dids.map((did) => (
                <TableRow
                  key={did.did}
                  className="border-b-white/5 hover:bg-white/5 transition-colors"
                >
                  <TableCell>
                    <span className="font-semibold text-primary/80">{did.alias}</span>
                  </TableCell>
                  <TableCell>
                    <span className="font-mono text-[10px] text-muted-foreground break-all">
                      {did.did}
                    </span>
                  </TableCell>
                  <TableCell>
                    <Badge variant="info" className="font-mono">
                      {did.type}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <Badge variant={did.default ? 'default' : 'info'}>
                      {did.default ? 'PRIMARY' : 'SECONDARY'}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <span className="text-xs text-muted-foreground">{did.keys?.length ?? 0}</span>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      </PageSection>
    </div>
  );
};

export default WalletInfo;
