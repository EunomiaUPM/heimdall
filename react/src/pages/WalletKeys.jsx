import { useState, useEffect } from 'react';
import { Key, Loader2, Plus, Trash2 } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { PageSection } from '@/components/layout/PageSection';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';

/** Kty/Crv may arrive as bare string, { Other: "..." } or null. */
const kindToString = (value) => {
  if (value == null) return '';
  if (typeof value === 'string') return value;
  if (typeof value === 'object') {
    if ('Other' in value) return String(value.Other);
    const keys = Object.keys(value);
    return keys[0] ?? '';
  }
  return String(value);
};

const NewKeyDialog = ({ onCreated }) => {
  const [open, setOpen] = useState(false);
  const [pem, setPem] = useState('');
  const [alias, setAlias] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState(null);

  const canSubmit = pem.trim().includes('BEGIN') && pem.trim().includes('END');

  const handleSubmit = async () => {
    setSubmitting(true);
    setError(null);
    try {
      const res = await fetch(`${apiUrl}/wallet/key`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pem, alias: alias || null }),
      });
      if (!res.ok) throw new Error('Failed to import key');
      setOpen(false);
      setPem('');
      setAlias('');
      await onCreated();
    } catch (err) {
      setError(err.message);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size="sm">
          <Plus className="h-3 w-3" />
          <span className="ml-1">Import Key</span>
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Import a private key</DialogTitle>
          <DialogDescription>
            Paste a PEM-encoded private key. The wallet derives kty/crv from the PEM headers.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          <div className="space-y-1">
            <Label className="text-xs">Alias (optional)</Label>
            <Input
              placeholder="my-signing-key"
              value={alias}
              onChange={(e) => setAlias(e.target.value)}
            />
          </div>
          <div className="space-y-1">
            <Label className="text-xs">PEM</Label>
            <textarea
              className="w-full font-mono text-[11px] bg-black/30 border border-white/10 rounded p-3 min-h-[200px] text-foreground"
              placeholder={'-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----'}
              value={pem}
              onChange={(e) => setPem(e.target.value)}
            />
            {!canSubmit && pem.trim().length > 0 && (
              <p className="text-[10px] text-amber-500">PEM must include BEGIN/END markers.</p>
            )}
          </div>
          {error && (
            <div className="p-2 rounded border border-destructive/30 bg-destructive/10 text-xs text-destructive">
              {error}
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => setOpen(false)}>
            Cancel
          </Button>
          <Button disabled={!canSubmit || submitting} onClick={handleSubmit}>
            {submitting ? (
              <Loader2 className="h-3 w-3 animate-spin mr-1" />
            ) : (
              <Plus className="h-3 w-3 mr-1" />
            )}
            Import
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

const WalletKeys = () => {
  const [keys, setKeys] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [deletingId, setDeletingId] = useState(null);

  const fetchKeys = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${apiUrl}/wallet/keys`);
      if (!res.ok) throw new Error('Failed to fetch keys');
      const data = await res.json();
      setKeys(data);
    } catch (err) {
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchKeys();
  }, []);

  const handleDelete = async (id) => {
    setDeletingId(id);
    try {
      const res = await fetch(`${apiUrl}/wallet/key/${encodeURIComponent(id)}`, {
        method: 'DELETE',
      });
      if (!res.ok) throw new Error('Failed to delete key');
      await fetchKeys();
    } catch (err) {
      setError(err);
    } finally {
      setDeletingId(null);
    }
  };

  if (loading) {
    return (
      <PageSection title="Keys">
        <Skeleton className="h-64 w-full rounded-2xl" />
      </PageSection>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchKeys} />;

  return (
    <div className="space-y-8 pb-20">
      <PageSection title="Keys" action={<NewKeyDialog onCreated={fetchKeys} />}>
        {keys.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 text-muted-foreground border border-dashed border-white/10 rounded-2xl bg-white/2">
            <Key className="h-12 w-12 opacity-10 mb-4" />
            <p className="text-sm font-medium">No keys stored yet.</p>
            <p className="text-xs opacity-60">Import a PEM-encoded private key to get started.</p>
          </div>
        ) : (
          <div className="rounded-md border border-white/10 bg-background-200/5">
            <Table className="text-sm">
              <TableHeader>
                <TableRow className="border-b-white/10 hover:bg-transparent">
                  <TableHead className="text-white/80">Alias</TableHead>
                  <TableHead className="text-white/80">ID</TableHead>
                  <TableHead className="text-white/80">Type</TableHead>
                  <TableHead className="text-white/80">Curve</TableHead>
                  <TableHead className="text-white/80">Created</TableHead>
                  <TableHead className="text-white/80">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {keys.map((k) => (
                  <TableRow key={k.id} className="border-b-white/5 hover:bg-white/5">
                    <TableCell>
                      <span className="font-semibold text-primary/80">{k.alias || '—'}</span>
                    </TableCell>
                    <TableCell>
                      <span className="font-mono text-[10px] text-muted-foreground break-all">
                        {k.id}
                      </span>
                    </TableCell>
                    <TableCell>
                      <Badge variant="info" className="font-mono">
                        {kindToString(k.kty)}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <span className="font-mono text-xs text-muted-foreground">
                        {kindToString(k.crv) || '—'}
                      </span>
                    </TableCell>
                    <TableCell>
                      <span className="text-xs text-muted-foreground">
                        {k.created_at ? new Date(k.created_at).toLocaleDateString() : '—'}
                      </span>
                    </TableCell>
                    <TableCell>
                      <Button
                        size="sm"
                        variant="ghost"
                        disabled={deletingId !== null}
                        onClick={() => handleDelete(k.id)}
                      >
                        {deletingId === k.id ? (
                          <Loader2 className="h-3 w-3 animate-spin" />
                        ) : (
                          <Trash2 className="h-3 w-3 text-destructive" />
                        )}
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        )}
      </PageSection>
    </div>
  );
};

export default WalletKeys;
