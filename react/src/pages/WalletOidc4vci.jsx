import { useState } from 'react';

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
    <div>
      <h2 style={{ color: '#ff00ff', marginBottom: '20px' }}>OIDC4VCI Issuance</h2>
      <div
        style={{
          border: '2px solid #ff00ff',
          padding: '25px',
          borderRadius: '8px',
          backgroundColor: 'rgba(26, 29, 53, 0.6)',
          boxShadow: '0 0 20px rgba(255, 0, 255, 0.3)',
        }}
      >
        <form onSubmit={handleProcess}>
          <div style={{ marginBottom: '20px' }}>
            <label
              htmlFor="uri"
              style={{
                display: 'block',
                color: '#ff00ff',
                marginBottom: '10px',
                fontSize: '1.1em',
              }}
            >
              Enter OIDC4VCI Credential URI:
            </label>
            <input
              id="uri"
              type="text"
              value={uri}
              onChange={(e) => setUri(e.target.value)}
              placeholder="openid-credential-offer://..."
              style={{
                width: '100%',
                padding: '12px',
                backgroundColor: 'rgba(10, 14, 39, 0.8)',
                border: '2px solid #ff00ff',
                borderRadius: '4px',
                color: '#e0e0e0',
                fontSize: '1em',
                outline: 'none',
                boxShadow: '0 0 10px rgba(255, 0, 255, 0.2)',
              }}
            />
          </div>
          <button
            type="submit"
            disabled={loading || !uri}
            style={{
              padding: '12px 30px',
              backgroundColor:
                loading || !uri ? 'rgba(255, 0, 255, 0.1)' : 'rgba(255, 0, 255, 0.2)',
              color: '#ff00ff',
              border: '2px solid #ff00ff',
              borderRadius: '4px',
              cursor: loading || !uri ? 'not-allowed' : 'pointer',
              fontSize: '1.1em',
              fontWeight: 'bold',
              boxShadow: '0 0 15px rgba(255, 0, 255, 0.4)',
              transition: 'all 0.3s ease',
            }}
            onMouseEnter={(e) => {
              if (!loading && uri) {
                e.currentTarget.style.backgroundColor = 'rgba(255, 0, 255, 0.3)';
                e.currentTarget.style.boxShadow = '0 0 25px rgba(255, 0, 255, 0.6)';
              }
            }}
            onMouseLeave={(e) => {
              if (!loading && uri) {
                e.currentTarget.style.backgroundColor = 'rgba(255, 0, 255, 0.2)';
                e.currentTarget.style.boxShadow = '0 0 15px rgba(255, 0, 255, 0.4)';
              }
            }}
          >
            {loading ? 'PROCESANDO...' : 'PROCESAR'}
          </button>
        </form>

        {error && (
          <div
            style={{
              marginTop: '20px',
              padding: '15px',
              border: '1px solid #ff0040',
              backgroundColor: 'rgba(255, 0, 64, 0.1)',
              color: '#ff0040',
              borderRadius: '4px',
            }}
          >
            <strong>Error:</strong> {error}
          </div>
        )}

        {response && (
          <div
            style={{
              marginTop: '20px',
              padding: '15px',
              border: '1px solid #00ff41',
              backgroundColor: 'rgba(0, 255, 65, 0.1)',
              color: '#00ff41',
              borderRadius: '4px',
              textAlign: 'center',
              fontWeight: 'bold',
            }}
          >
            ✓ OIDC4VCI Processed Successfully!
          </div>
        )}
      </div>
    </div>
  );
};

export default WalletOidc4vci;
