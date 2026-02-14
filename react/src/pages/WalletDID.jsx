import { useState, useEffect } from 'react';

const WalletDID = () => {
  const [didDocument, setDidDocument] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchDID = async () => {
      try {
        const response = await fetch(`${apiUrl}/wallet/did.json`);
        if (!response.ok) {
          throw new Error('Failed to fetch DID document');
        }
        const data = await response.json();
        setDidDocument(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching DID:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchDID();
  }, [apiUrl]);

  if (loading) return <div className="text-brand-sky">Loading DID document...</div>;
  if (error) return <div className="text-danger">Error: {error}</div>;

  return (
    <div>
      <h2 className="text-2xl font-bold text-brand-sky mb-4">DID Document</h2>
      <div className="rounded-lg border border-brand-purple bg-background/60 p-6 shadow-lg shadow-brand-purple/20 text-left">
        <pre className="text-muted-foreground whitespace-pre-wrap break-all font-mono text-sm leading-relaxed">
          {JSON.stringify(didDocument, null, 2)}
        </pre>
      </div>
    </div>
  );
};

export default WalletDID;
