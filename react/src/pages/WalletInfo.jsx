import { useState, useEffect } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import BooleanBadge from '../components/BooleanBadge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { cn } from '@/lib/utils';
import { ChevronRight, FileJson, Copy, Check } from 'lucide-react';

const DidDocItem = ({ did }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  
  let formattedDoc = did.document;
  try {
    const parsed = typeof did.document === 'string' ? JSON.parse(did.document) : did.document;
    formattedDoc = JSON.stringify(parsed, null, 2);
  } catch (e) {
    // leave as is if not JSON
  }

  const handleCopy = (e) => {
    e.stopPropagation();
    navigator.clipboard.writeText(formattedDoc);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="group border border-brand-purple/20 rounded-xl overflow-hidden bg-background/50 transition-all hover:bg-background/60 shadow-md mb-4">
      <button 
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between p-4 text-left transition-colors"
      >
        <div className="flex items-center gap-3">
           <div className={cn("transition-transform duration-200", isOpen ? "rotate-90 text-brand-purple" : "text-muted-foreground")}>
             <ChevronRight className="h-5 w-5" />
           </div>
           <span className="text-sm font-semibold text-foreground">{did.alias}</span>
           <span className="text-xs text-muted-foreground/60 font-mono truncate max-w-[200px] md:max-w-md">{did.did}</span>
        </div>
      </button>
      {isOpen && (
        <div className="p-4 pt-0">
          <div className="bg-black/30 rounded-xl border border-brand-purple/10 overflow-hidden">
             <div className="flex items-center justify-between px-4 py-2 border-b border-brand-purple/10 bg-black/20">
                <span className="text-[10px] font-bold uppercase tracking-widest text-brand-purple/70 flex items-center gap-2">
                  <FileJson className="h-3 w-3" />
                  JSON Document
                </span>
                <button 
                  onClick={handleCopy}
                  className="flex items-center gap-1.5 text-[10px] font-mono text-muted-foreground hover:text-foreground transition-colors"
                >
                  {copied ? <Check className="h-3 w-3 text-success" /> : <Copy className="h-3 w-3" />}
                  {copied ? "Copied!" : "Copy"}
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

  useEffect(() => {
    const fetchWalletInfo = async () => {
      try {
        const response = await fetch(`${apiUrl}/wallet/info`);
        if (!response.ok) {
          throw new Error('Failed to fetch wallet info');
        }
        const data = await response.json();
        setWalletInfo(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching wallet info:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchWalletInfo();
  }, [apiUrl]);

  if (loading) return <div className="text-brand-sky">Loading wallet info...</div>;
  if (error) return <div className="text-danger">Error: {error}</div>;

  return (
    <div className="text-left w-full">
      <h2 className="text-2xl font-bold text-brand-sky mb-6 text-center">Wallet Information</h2>

      {/* Wallet Details Card */}
      <div className="rounded-lg border border-brand-sky bg-background/60 p-6 shadow-lg shadow-brand-sky/20 mb-8 items-start">
        <div className="grid grid-cols-1 gap-4">
          <p>
            <strong className="text-brand-sky">ID:</strong>{' '}
            <span className="text-muted-foreground">{walletInfo.id}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Name:</strong>{' '}
            <span className="text-muted-foreground">{walletInfo.name}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Created On:</strong>{' '}
            <span className="text-muted-foreground">{walletInfo.createdOn}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Added On:</strong>{' '}
            <span className="text-muted-foreground">{walletInfo.addedOn}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Permission:</strong>{' '}
            <span className="text-muted-foreground">{walletInfo.permission}</span>
          </p>
        </div>
      </div>

      {/* DIDs Table */}
      <h3 className="text-xl font-bold text-brand-purple mb-4">DIDs</h3>
      <div className="rounded-md border border-brand-purple bg-background/50 shadow-md shadow-brand-purple/20 mb-8 overflow-hidden">
        <Table>
          <TableHeader>
            <TableRow className="border-b-brand-purple/50 bg-brand-purple/10 hover:bg-brand-purple/10">
              <TableHead className="text-brand-purple font-bold">DID</TableHead>
              <TableHead className="text-brand-purple font-bold">Alias</TableHead>
              <TableHead className="text-brand-purple font-bold">Key ID</TableHead>
              <TableHead className="text-brand-purple font-bold">Default</TableHead>
              <TableHead className="text-brand-purple font-bold">Created On</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {walletInfo.dids.map((did, index) => (
              <TableRow key={index} className="border-b-brand-purple/20 hover:bg-brand-purple/5">
                <TableCell className="text-muted-foreground break-all">{did.did}</TableCell>
                <TableCell className="text-muted-foreground">{did.alias}</TableCell>
                <TableCell className="text-muted-foreground break-all">{did.keyId}</TableCell>
                <TableCell>
                  <BooleanBadge value={did.default} />
                </TableCell>
                <TableCell className="text-muted-foreground">{did.createdOn}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      {/* DID Document Previews */}
      <h3 className="text-xl font-bold text-brand-purple mb-4">DID Documents</h3>
      {walletInfo.dids.map((did, index) => (
        <DidDocItem key={index} did={did} />
      ))}
    </div>
  );
};

export default WalletInfo;
