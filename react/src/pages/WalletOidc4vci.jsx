import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';

const WalletOidc4vci = () => {
  const [uri, setUri] = useState('');
  const [loading, setLoading] = useState(false);
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  const handleProcess = async (e) => {
    e.preventDefault();
    if (!uri) return;

    setLoading(true);
    setResponse(null);
    setError(null);

    try {
      const res = await fetch(`${apiUrl}/wallet/oidc4vci`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ uri }),
      });

      if (!res.ok) {
        throw new Error(`Failed to process OIDC4VCI: ${res.statusText}`);
      }

      const data = await res.json();
      setResponse(data);
    } catch (err) {
      console.error('Error processing OIDC4VCI:', err);
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-3xl mx-auto">
      <h2 className="text-2xl font-bold text-brand-purple mb-6 text-center">OIDC4VCI Issuance</h2>
      <div className="rounded-lg border border-brand-purple bg-background/60 p-8 shadow-lg shadow-brand-purple/20">
        <form onSubmit={handleProcess} className="space-y-6">
          <div className="grid w-full items-center gap-3">
            <Label htmlFor="uri" className="text-brand-purple text-lg">
              Enter OIDC4VCI Credential URI:
            </Label>
            <Input
              id="uri"
              type="text"
              value={uri}
              onChange={(e) => setUri(e.target.value)}
              placeholder="openid-credential-offer://..."
              className="border-brand-purple focus-visible:ring-brand-purple/50 bg-background/80"
            />
          </div>
          <Button
            type="submit"
            disabled={loading || !uri}
            className="w-full bg-brand-purple/20 text-brand-purple border border-brand-purple hover:bg-brand-purple/30 shadow-lg shadow-brand-purple/20 font-bold text-lg h-12"
          >
            {loading ? 'PROCESANDO...' : 'PROCESAR'}
          </Button>
        </form>

        {error && (
          <div className="mt-6 p-4 border border-danger bg-danger/10 text-danger rounded-md">
            <strong>Error:</strong> {error}
          </div>
        )}

        {response && (
          <div className="mt-6 p-4 border border-success bg-success/10 text-success rounded-md text-center font-bold">
            ✓ OIDC4VCI Processed Successfully!
          </div>
        )}
      </div>
    </div>
  );
};

export default WalletOidc4vci;
