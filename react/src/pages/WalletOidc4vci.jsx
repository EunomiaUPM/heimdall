import { useState } from 'react';
import { FileText, Loader2 } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { PageSection } from '@/components/layout/PageSection';

const WalletOidc4vci = () => {
  const [uri, setUri] = useState('');
  const [loading, setLoading] = useState(false);
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);

  const handleProcess = async (e) => {
    e.preventDefault();
    if (!uri) return;
    setLoading(true);
    setResponse(null);
    setError(null);
    try {
      const res = await fetch(`${apiUrl}/wallet/oidc4vci`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ uri }),
      });
      if (!res.ok) throw new Error(`Failed to process OIDC4VCI: ${res.statusText}`);
      setResponse({ success: true });
      setUri('');
    } catch (err) {
      console.error('Error processing OIDC4VCI:', err);
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto py-8">
      <PageSection title="Credential Issuance (OIDC4VCI)">
        <div className="bg-white/5 border border-white/10 rounded-2xl p-10 backdrop-blur-md shadow-2xl space-y-8">
          <div className="text-center space-y-2">
            <FileText className="h-12 w-12 text-primary mx-auto mb-2" />
            <p className="text-sm text-muted-foreground">
              Paste a Credential Offer URI to receive new verifiable credentials.
            </p>
          </div>

          <form onSubmit={handleProcess} className="space-y-6">
            <div className="space-y-2">
              <Label
                htmlFor="oidc4vci-uri"
                className="text-xs uppercase tracking-widest text-muted-foreground/60 font-bold"
              >
                Credential Offer URI
              </Label>
              <Input
                id="oidc4vci-uri"
                placeholder="openid-credential-offer://…"
                value={uri}
                onChange={(e) => setUri(e.target.value)}
                className="bg-black/30 border-white/10 h-12 focus:ring-primary/50 text-sm font-mono"
              />
            </div>

            <Button
              type="submit"
              disabled={loading || !uri}
              className="w-full h-12 text-base font-bold shadow-lg shadow-primary/10 transition-all active:scale-[0.98]"
            >
              {loading ? (
                <span className="flex items-center gap-2">
                  <Loader2 className="h-4 w-4 animate-spin" />
                  Issuing…
                </span>
              ) : (
                'Request Credential'
              )}
            </Button>
          </form>

          {response && (
            <div className="p-4 rounded-lg bg-green-500/10 border border-green-500/20 text-green-400 text-sm text-center font-medium">
              ✓ Credential issuance completed successfully
            </div>
          )}

          {error && (
            <div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-sm text-center font-mono">
              Error: {error}
            </div>
          )}
        </div>
      </PageSection>
    </div>
  );
};

export default WalletOidc4vci;
