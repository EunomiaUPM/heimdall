import { useState, useEffect } from 'react';
import {
  Check,
  ChevronRight,
  Copy,
  FileJson,
  Key,
  Loader2,
  Plus,
  Star,
  Trash2,
} from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { cn } from '@/lib/utils';
import { Skeleton } from '@/components/ui/skeleton';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { PageSection } from '@/components/layout/PageSection';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';

const SERVICE_TYPES = ['AuthorizationServer', 'CredentialIssuer', 'FederatedCatalog'];

/** Wrapper around fetch that throws on non-2xx. */
async function api(path, init = {}) {
  const res = await fetch(`${apiUrl}${path}`, {
    headers: { 'Content-Type': 'application/json', ...(init.headers || {}) },
    ...init,
  });
  if (!res.ok) {
    let msg = res.statusText;
    try {
      const body = await res.json();
      if (body?.reason) msg = body.reason;
      else if (body?.message) msg = body.message;
    } catch {}
    throw new Error(msg || `HTTP ${res.status}`);
  }
  if (res.status === 204) return null;
  const ct = res.headers.get('content-type') || '';
  return ct.includes('application/json') ? res.json() : res.text();
}

const WalletDID = () => {
  const [info, setInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [actionError, setActionError] = useState(null);
  const [settingDefaultId, setSettingDefaultId] = useState(null);

  const fetchInfo = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await api('/wallet/info');
      setInfo(data);
    } catch (err) {
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchInfo();
  }, []);

  const setDefault = async (id) => {
    setSettingDefaultId(id);
    setActionError(null);
    try {
      await api(`/wallet/did/${encodeURIComponent(id)}/default`, { method: 'POST' });
      await fetchInfo();
    } catch (err) {
      setActionError(err.message);
    } finally {
      setSettingDefaultId(null);
    }
  };

  if (loading) {
    return (
      <PageSection title="DIDs">
        <Skeleton className="h-64 w-full rounded-2xl" />
      </PageSection>
    );
  }
  if (error) return <GeneralErrorComponent error={error} reset={fetchInfo} />;

  const dids = info?.dids ?? [];

  return (
    <div className="space-y-8 pb-20">
      {actionError && (
        <div className="p-3 rounded border border-destructive/30 bg-destructive/10 text-sm text-destructive">
          {actionError}
        </div>
      )}

      <PageSection title="DIDs" action={<NewDidDialog onCreated={fetchInfo} />}>
        {dids.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 text-muted-foreground border border-dashed border-white/10 rounded-2xl bg-white/2">
            <Key className="h-12 w-12 opacity-10 mb-4" />
            <p className="text-sm font-medium">No DIDs registered yet.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {dids.map((d) => (
              <DidPanel
                key={d.did}
                did={d}
                onSetDefault={() => setDefault(d.id)}
                isSettingDefault={settingDefaultId === d.id}
                onChanged={fetchInfo}
                reportError={setActionError}
              />
            ))}
          </div>
        )}
      </PageSection>
    </div>
  );
};

const DidPanel = ({ did, onSetDefault, isSettingDefault, onChanged, reportError }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  const [busy, setBusy] = useState(null);

  const isWeb = did.type === 'Web';
  const formattedDoc = JSON.stringify(did.did_document, null, 2);

  const run = async (kind, fn) => {
    setBusy(kind);
    try {
      await fn();
      await onChanged();
    } catch (err) {
      reportError(err.message);
    } finally {
      setBusy(null);
    }
  };

  const handleCopy = (e) => {
    e.stopPropagation();
    navigator.clipboard.writeText(formattedDoc);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="group border border-white/10 rounded-xl overflow-hidden bg-white/[0.02] transition-all hover:bg-white/[0.04]">
      <div className="w-full flex items-center justify-between p-4 gap-3">
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="flex items-center gap-3 flex-1 min-w-0 text-left"
        >
          <div
            className={cn(
              'transition-transform duration-200 shrink-0',
              isOpen ? 'rotate-90 text-primary' : 'text-muted-foreground',
            )}
          >
            <ChevronRight className="h-5 w-5" />
          </div>
          <span className="text-sm font-semibold">{did.alias}</span>
          {did.default && <Badge variant="default">PRIMARY</Badge>}
          <Badge variant="info" className="font-mono text-[10px]">
            {did.type}
          </Badge>
          <span className="text-xs text-muted-foreground/60 font-mono truncate">{did.did}</span>
        </button>

        <div className="flex items-center gap-2 shrink-0">
          {!did.default && (
            <Button size="sm" variant="outline" disabled={isSettingDefault} onClick={onSetDefault}>
              {isSettingDefault ? (
                <Loader2 className="h-3 w-3 animate-spin" />
              ) : (
                <Star className="h-3 w-3" />
              )}
              <span className="ml-1">Set default</span>
            </Button>
          )}
          <Button
            size="sm"
            variant="ghost"
            disabled={did.default || busy === 'delete'}
            onClick={() =>
              run('delete', () =>
                api(`/wallet/did/${encodeURIComponent(did.id)}`, { method: 'DELETE' }),
              )
            }
            title={did.default ? 'Cannot delete the active default DID' : 'Delete this DID'}
          >
            {busy === 'delete' ? (
              <Loader2 className="h-3 w-3 animate-spin" />
            ) : (
              <Trash2 className="h-3 w-3 text-destructive" />
            )}
          </Button>
        </div>
      </div>

      {isOpen && (
        <div className="p-4 pt-0 space-y-6">
          <div>
            <div className="flex items-center gap-2 mb-3">
              <Key className="h-4 w-4 text-primary" />
              <h5 className="text-xs uppercase tracking-widest font-bold text-foreground/70">
                Attached Keys
              </h5>
            </div>
            <div className="space-y-2">
              {did.keys.map((k) => {
                const isDefault = k.internal === did.default_key?.internal;
                return (
                  <div
                    key={`${k.internal}-${k.fragment}`}
                    className="flex items-center justify-between gap-3 bg-white/5 rounded-lg p-3 border border-white/5"
                  >
                    <div className="min-w-0 flex-1 space-y-1">
                      <div className="font-mono text-xs text-foreground/80 truncate">
                        #{k.fragment}
                      </div>
                      <div className="font-mono text-[10px] text-muted-foreground truncate">
                        {k.internal}
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      {isDefault ? (
                        <Badge variant="default" className="text-[10px]">
                          DEFAULT
                        </Badge>
                      ) : (
                        <Button
                          size="sm"
                          variant="ghost"
                          disabled={!isWeb || busy === 'default-key'}
                          onClick={() =>
                            run('default-key', () =>
                              api(
                                `/wallet/did/${encodeURIComponent(did.id)}/key/default/${encodeURIComponent(k.internal)}`,
                                { method: 'POST' },
                              ),
                            )
                          }
                          title={
                            isWeb
                              ? 'Set as default signing key'
                              : 'did:jwk has a single key — cannot change default'
                          }
                        >
                          <Star className="h-3 w-3" />
                        </Button>
                      )}
                      <Button
                        size="sm"
                        variant="ghost"
                        disabled={!isWeb || did.keys.length === 1 || busy === 'remove'}
                        onClick={() =>
                          run('remove', () =>
                            api(
                              `/wallet/did/${encodeURIComponent(did.id)}/key/${encodeURIComponent(k.internal)}`,
                              { method: 'DELETE' },
                            ),
                          )
                        }
                        title={
                          !isWeb
                            ? 'did:jwk keys must be removed by deleting the DID itself'
                            : did.keys.length === 1
                              ? 'Cannot remove the only key of a DID'
                              : 'Remove key from DID'
                        }
                      >
                        <Trash2 className="h-3 w-3 text-destructive" />
                      </Button>
                    </div>
                  </div>
                );
              })}
            </div>

            {isWeb ? (
              <AttachKeyForm
                excludeIds={did.keys.map((k) => k.internal)}
                onSubmit={(keyId) =>
                  run('add', () =>
                    api(
                      `/wallet/did/${encodeURIComponent(did.id)}/key/${encodeURIComponent(keyId)}`,
                      { method: 'POST' },
                    ),
                  )
                }
                isSubmitting={busy === 'add'}
              />
            ) : (
              <p className="mt-3 text-[10px] text-muted-foreground italic">
                did:jwk DIDs hold a single key bound to their identifier. Create a new DID instead.
              </p>
            )}
          </div>

          <div>
            <div className="bg-black/40 rounded-xl border border-white/5 overflow-hidden">
              <div className="flex items-center justify-between px-4 py-2 border-b border-white/5 bg-white/[0.02]">
                <span className="text-[10px] font-bold uppercase tracking-widest text-primary/60 flex items-center gap-2">
                  <FileJson className="h-3 w-3" />
                  DID Document
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
        </div>
      )}
    </div>
  );
};

const AttachKeyForm = ({ excludeIds, onSubmit, isSubmitting }) => {
  const [allKeys, setAllKeys] = useState([]);
  const [keyId, setKeyId] = useState('');

  useEffect(() => {
    api('/wallet/keys').then(setAllKeys).catch(() => {});
  }, []);

  const candidates = allKeys.filter((k) => !excludeIds.includes(k.id));

  return (
    <div className="mt-4 space-y-2">
      <Label className="text-[10px] uppercase tracking-widest text-muted-foreground/70">
        Attach existing key
      </Label>
      <div className="flex items-center gap-2">
        <select
          className="flex-1 bg-black/30 border border-white/10 rounded px-3 py-2 text-xs font-mono text-foreground"
          value={keyId}
          onChange={(e) => setKeyId(e.target.value)}
        >
          <option value="">— Choose a key —</option>
          {candidates.map((k) => (
            <option key={k.id} value={k.id}>
              {k.alias ? `${k.alias} (${k.id})` : k.id}
            </option>
          ))}
        </select>
        <Button
          size="sm"
          disabled={!keyId || isSubmitting}
          onClick={() => {
            onSubmit(keyId);
            setKeyId('');
          }}
        >
          {isSubmitting ? (
            <Loader2 className="h-3 w-3 animate-spin" />
          ) : (
            <Plus className="h-3 w-3" />
          )}
          <span className="ml-1">Add key</span>
        </Button>
      </div>
      {candidates.length === 0 && (
        <p className="text-[10px] text-muted-foreground italic">
          No spare keys available. Create one from the Keys tab.
        </p>
      )}
    </div>
  );
};

const NewDidDialog = ({ onCreated }) => {
  const [open, setOpen] = useState(false);
  const [kind, setKind] = useState('jwk');

  const [alias, setAlias] = useState('');
  const [domain, setDomain] = useState('');
  const [path, setPath] = useState('');
  const [port, setPort] = useState('');
  const [keysId, setKeysId] = useState([]);
  const [services, setServices] = useState([]);
  const [allKeys, setAllKeys] = useState([]);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!open) return;
    api('/wallet/keys').then(setAllKeys).catch(() => {});
  }, [open]);

  const reset = () => {
    setKind('jwk');
    setAlias('');
    setDomain('');
    setPath('');
    setPort('');
    setKeysId([]);
    setServices([]);
    setError(null);
  };

  const toggleKey = (id) => {
    if (kind === 'jwk') {
      setKeysId([id]);
    } else {
      setKeysId((curr) => (curr.includes(id) ? curr.filter((k) => k !== id) : [...curr, id]));
    }
  };

  const addService = () => setServices((s) => [...s, { type: '', serviceEndpoint: '' }]);
  const removeService = (idx) => setServices((s) => s.filter((_, i) => i !== idx));
  const patchService = (idx, patch) =>
    setServices((s) => s.map((v, i) => (i === idx ? { ...v, ...patch } : v)));

  const canSubmit =
    alias.trim() &&
    keysId.length > 0 &&
    (kind === 'jwk' || domain.trim() !== '') &&
    !submitting;

  const handleSubmit = async () => {
    setSubmitting(true);
    setError(null);
    try {
      const builder =
        kind === 'web'
          ? {
              Web: {
                domain: domain.trim(),
                path: path.trim() || null,
                port: port.trim() || null,
              },
            }
          : { Jwk: { pem: '' } };
      const cleanedServices = services
        .map((s) => ({
          ...(s.id ? { id: s.id } : {}),
          type: s.type.trim(),
          serviceEndpoint: s.serviceEndpoint.trim(),
        }))
        .filter((s) => s.type && s.serviceEndpoint);
      await api('/wallet/did', {
        method: 'POST',
        body: JSON.stringify({
          builder,
          keys_id: keysId,
          alias,
          service: cleanedServices.length > 0 ? cleanedServices : null,
        }),
      });
      setOpen(false);
      reset();
      await onCreated();
    } catch (err) {
      setError(err.message);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Dialog
      open={open}
      onOpenChange={(o) => {
        setOpen(o);
        if (!o) reset();
      }}
    >
      <DialogTrigger asChild>
        <Button size="sm">
          <Plus className="h-3 w-3" />
          <span className="ml-1">New DID</span>
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-2xl max-h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Register a new DID</DialogTitle>
          <DialogDescription>
            Pick a method, attach an existing wallet key, optionally declare service endpoints.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-5">
          <div className="space-y-1">
            <Label className="text-xs">DID method</Label>
            <div className="flex gap-2">
              <Button
                size="sm"
                variant={kind === 'jwk' ? 'default' : 'outline'}
                onClick={() => {
                  setKind('jwk');
                  setKeysId((curr) => curr.slice(0, 1));
                }}
              >
                did:jwk
              </Button>
              <Button
                size="sm"
                variant={kind === 'web' ? 'default' : 'outline'}
                onClick={() => setKind('web')}
              >
                did:web
              </Button>
            </div>
            <p className="text-[10px] text-muted-foreground italic">
              {kind === 'jwk'
                ? "did:jwk derives the identifier from a single key's public material."
                : 'did:web is hosted at the given URL; you can bind multiple keys.'}
            </p>
          </div>

          <div className="space-y-1">
            <Label className="text-xs">Alias</Label>
            <Input
              placeholder={kind === 'jwk' ? 'my-jwk-did' : 'my-web-did'}
              value={alias}
              onChange={(e) => setAlias(e.target.value)}
            />
          </div>

          {kind === 'web' && (
            <>
              <div className="space-y-1">
                <Label className="text-xs">Domain</Label>
                <Input
                  placeholder="example.com"
                  value={domain}
                  onChange={(e) => setDomain(e.target.value)}
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="space-y-1">
                  <Label className="text-xs">Path (optional)</Label>
                  <Input value={path} onChange={(e) => setPath(e.target.value)} />
                </div>
                <div className="space-y-1">
                  <Label className="text-xs">Port (optional)</Label>
                  <Input value={port} onChange={(e) => setPort(e.target.value)} />
                </div>
              </div>
            </>
          )}

          <div className="space-y-1">
            <Label className="text-xs">
              {kind === 'jwk' ? 'Bind key (single)' : 'Attach keys (one or more)'}
            </Label>
            {allKeys.length === 0 ? (
              <p className="text-[10px] text-muted-foreground italic">
                No keys available. Create one from the Keys tab.
              </p>
            ) : (
              <div className="space-y-1 max-h-40 overflow-y-auto border border-white/10 rounded-lg p-2">
                {allKeys.map((k) => (
                  <label
                    key={k.id}
                    className="flex items-center gap-2 p-1 text-xs cursor-pointer hover:bg-white/5 rounded"
                  >
                    <input
                      type={kind === 'jwk' ? 'radio' : 'checkbox'}
                      name="did-keys"
                      checked={keysId.includes(k.id)}
                      onChange={() => toggleKey(k.id)}
                    />
                    <span className="font-mono">{k.alias || k.id}</span>
                    <span className="font-mono text-[10px] text-muted-foreground truncate">
                      {k.id}
                    </span>
                  </label>
                ))}
              </div>
            )}
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label className="text-xs">Services (optional)</Label>
              <Button size="sm" variant="ghost" onClick={addService}>
                <Plus className="h-3 w-3" />
                <span className="ml-1">Add service</span>
              </Button>
            </div>
            {services.length === 0 ? (
              <p className="text-[10px] text-muted-foreground italic">
                Declare any service endpoint to expose in the DID document.
              </p>
            ) : (
              <div className="space-y-2">
                {services.map((svc, idx) => (
                  <div
                    key={idx}
                    className="border border-white/10 rounded-lg p-3 space-y-2 bg-white/[0.02]"
                  >
                    <div className="flex items-center gap-2">
                      <div className="flex-1 space-y-1">
                        <Label className="text-[10px] text-muted-foreground">Type</Label>
                        <input
                          list={`svc-types-${idx}`}
                          className="w-full bg-black/30 border border-white/10 rounded px-2 py-1.5 text-xs font-mono text-foreground"
                          placeholder="AuthorizationServer / CredentialIssuer / ..."
                          value={svc.type}
                          onChange={(e) => patchService(idx, { type: e.target.value })}
                        />
                        <datalist id={`svc-types-${idx}`}>
                          {SERVICE_TYPES.map((t) => (
                            <option key={t} value={t} />
                          ))}
                        </datalist>
                      </div>
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => removeService(idx)}
                        className="mt-4"
                      >
                        <Trash2 className="h-3 w-3 text-destructive" />
                      </Button>
                    </div>
                    <div className="space-y-1">
                      <Label className="text-[10px] text-muted-foreground">Service Endpoint</Label>
                      <Input
                        placeholder="https://example.com/oidc"
                        value={svc.serviceEndpoint}
                        onChange={(e) => patchService(idx, { serviceEndpoint: e.target.value })}
                      />
                    </div>
                  </div>
                ))}
              </div>
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
          <Button disabled={!canSubmit} onClick={handleSubmit}>
            {submitting ? (
              <Loader2 className="h-3 w-3 animate-spin mr-1" />
            ) : (
              <Plus className="h-3 w-3 mr-1" />
            )}
            Register DID
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default WalletDID;
