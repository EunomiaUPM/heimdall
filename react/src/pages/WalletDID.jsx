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

  if (loading) return <div style={{ color: '#00f0ff' }}>Loading DID document...</div>;
  if (error) return <div style={{ color: '#ff0040' }}>Error: {error}</div>;

  return (
    <div>
      <h2 style={{ color: '#00f0ff', marginBottom: '20px' }}>DID Document</h2>
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
          {JSON.stringify(didDocument, null, 2)}
        </pre>
      </div>
    </div>
  );
};

export default WalletDID;
