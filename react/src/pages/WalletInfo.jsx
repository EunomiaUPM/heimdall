import { useState, useEffect } from 'react';
import BooleanBadge from '../components/BooleanBadge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';

const WalletInfo = () => {
  const [walletInfo, setWalletInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

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
        <details
          key={index}
          className="group rounded-lg border border-brand-purple bg-background/60 p-4 shadow-md shadow-brand-purple/20 mb-4 open:bg-background/80 transition-all"
        >
          <summary className="text-brand-purple cursor-pointer font-bold mb-2 group-open:mb-4 select-none hover:text-brand-purple/80 break-all">
            {did.alias} - {did.did}
          </summary>
          <pre className="text-muted-foreground whitespace-pre-wrap break-all font-mono text-xs leading-relaxed bg-black/40 p-4 rounded border border-brand-purple/30">
            {did.document}
          </pre>
        </details>
      ))}
    </div>
  );
};

export default WalletInfo;
