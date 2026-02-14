import { useState, useEffect } from 'react';

const WalletCredentials = () => {
  const [credentials, setCredentials] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchCredentials = async () => {
      try {
        const response = await fetch(`${apiUrl}/wallet/vcs`);
        if (!response.ok) {
          throw new Error('Failed to fetch credentials');
        }
        const data = await response.json();
        setCredentials(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching credentials:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchCredentials();
  }, [apiUrl]);

  if (loading) return <div className="text-brand-sky">Loading credentials...</div>;
  if (error) return <div className="text-danger">Error: {error}</div>;

  const hasCredentials =
    credentials &&
    (Array.isArray(credentials) ? credentials.length > 0 : Object.keys(credentials).length > 0);

  return (
    <div>
      <h2 className="text-2xl font-bold text-brand-sky mb-4">Verifiable Credentials</h2>

      {!hasCredentials ? (
        <div className="rounded-lg border border-brand-purple bg-background/60 p-8 shadow-lg shadow-brand-purple/20 text-center">
          <p className="text-muted-foreground text-lg">No credentials found</p>
        </div>
      ) : (
        <div className="flex flex-col gap-6">
          {(Array.isArray(credentials) ? credentials : [credentials]).map((vc, index) => (
            <div
              key={index}
              className="rounded-lg border border-brand-purple bg-background/60 p-6 shadow-lg shadow-brand-purple/20 text-left"
            >
              <pre className="text-muted-foreground whitespace-pre-wrap break-all font-mono text-sm leading-relaxed">
                {JSON.stringify(vc, null, 2)}
              </pre>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default WalletCredentials;
