import { useState, useEffect } from 'react';
import { ChevronRight, FileJson, Copy, Check } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { cn } from '@/lib/utils';
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

const DidDocItem = ({ did }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);

  let formattedDoc = did.document;
  try {
    const parsed = typeof did.document === 'string' ? JSON.parse(did.document) : did.document;
    formattedDoc = JSON.stringify(parsed, null, 2);
  } catch (e) {
    /* leave as-is if not JSON */
  }

  const handleCopy = (e) => {
    e.stopPropagation();
    navigator.clipboard.writeText(formattedDoc);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="group border border-white/10 rounded-xl overflow-hidden bg-white/[0.02] transition-all hover:bg-white/[0.04]">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between p-4 text-left transition-colors"
      >
        <div className="flex items-center gap-3">
          <div
            className={cn(
              'transition-transform duration-200',
              isOpen ? 'rotate-90 text-primary' : 'text-muted-foreground',
            )}
          >
            <ChevronRight className="h-5 w-5" />
          </div>
          <span className="text-sm font-semibold">{did.alias}</span>
          <span className="text-xs text-muted-foreground/60 font-mono truncate max-w-[200px] md:max-w-md">
            {did.did}
          </span>
        </div>
      </button>
      {isOpen && (
        <div className="p-4 pt-0">
          <div className="bg-black/40 rounded-xl border border-white/5 overflow-hidden">
            <div className="flex items-center justify-between px-4 py-2 border-b border-white/5 bg-white/[0.02]">
              <span className="text-[10px] font-bold uppercase tracking-widest text-primary/60 flex items-center gap-2">
                <FileJson className="h-3 w-3" />
                JSON Document
              </span>
              <button
                onClick={handleCopy}
                className="flex items-center gap-1.5 text-[10px] font-mono text-muted-foreground hover:text-primary transition-colors"
              >
                {copied ? (
                  <Check className="h-3 w-3 text-green-500" />
                ) : (
                  <Copy className="h-3 w-3" />
                )}
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>
            <div className="p-4 overflow-x-auto">
              <pre className="font-mono text-[11px] text-muted-foreground/90 whitespace-pre-wrap break-all leading-relaxed">
                {formattedDoc}
              </pre>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

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
              { label: 'Created On', value: walletInfo.createdOn },
              { label: 'Permission Level', value: walletInfo.permission },
            ]}
          />
        </div>
      </PageSection>

      <PageSection title="Associated DIDs">
        <div className="rounded-md border border-white/10 bg-background-200/5">
          <Table className="text-sm">
            <TableHeader>
              <TableRow className="border-b-white/10 hover:bg-transparent">
                <TableHead className="text-white/80">Alias</TableHead>
                <TableHead className="text-white/80">DID</TableHead>
                <TableHead className="text-white/80">Default</TableHead>
                <TableHead className="text-white/80">Created</TableHead>
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
                    <Badge variant={did.default ? 'default' : 'info'}>
                      {did.default ? 'PRIMARY' : 'SECONDARY'}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-muted-foreground">{did.createdOn}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      </PageSection>

      <PageSection title="DID Documents Details">
        <div className="space-y-4">
          {walletInfo.dids.map((did) => (
            <DidDocItem key={did.did} did={did} />
          ))}
        </div>
      </PageSection>
    </div>
  );
};

export default WalletInfo;
