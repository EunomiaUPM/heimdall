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

  if (loading) return <div style={{ color: '#00f0ff' }}>Loading credentials...</div>;
  if (error) return <div style={{ color: '#ff0040' }}>Error: {error}</div>;

  const hasCredentials =
    credentials &&
    (Array.isArray(credentials) ? credentials.length > 0 : Object.keys(credentials).length > 0);

  return (
    <div>
      <h2 style={{ color: '#00f0ff', marginBottom: '20px' }}>Verifiable Credentials</h2>

      {!hasCredentials ? (
        <div
          style={{
            border: '2px solid #ff00ff',
            padding: '30px',
            borderRadius: '8px',
            backgroundColor: 'rgba(26, 29, 53, 0.6)',
            boxShadow: '0 0 20px rgba(255, 0, 255, 0.3)',
            textAlign: 'center',
          }}
        >
          <p style={{ color: '#e0e0e0', fontSize: '1.1em' }}>No credentials found</p>
        </div>
      ) : (
        <div
          style={{
            border: '2px solid #ff00ff',
            padding: '20px',
            borderRadius: '8px',
            backgroundColor: 'rgba(26, 29, 53, 0.6)',
            boxShadow: '0 0 20px rgba(255, 0, 255, 0.3)',
          }}
        >
          <pre
            style={{
              color: '#e0e0e0',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
              margin: 0,
              fontFamily: 'Courier New, monospace',
              fontSize: '0.9em',
              lineHeight: '1.5',
            }}
          >
            {JSON.stringify(credentials, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
};

export default WalletCredentials;
