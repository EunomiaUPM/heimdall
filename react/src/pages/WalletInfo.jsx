import { useState, useEffect } from 'react';
import BooleanBadge from '../components/BooleanBadge';

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

  if (loading) return <div style={{ color: '#00f0ff' }}>Loading wallet info...</div>;
  if (error) return <div style={{ color: '#ff0040' }}>Error: {error}</div>;

  return (
    <div>
      <h2 style={{ color: '#00f0ff', marginBottom: '20px' }}>Wallet Information</h2>

      {/* Wallet Details Card */}
      <div
        style={{
          border: '2px solid #00f0ff',
          padding: '25px',
          borderRadius: '8px',
          marginBottom: '30px',
          backgroundColor: 'rgba(26, 29, 53, 0.6)',
          boxShadow: '0 0 20px rgba(0, 240, 255, 0.3)',
        }}
      >
        <p>
          <strong style={{ color: '#00f0ff' }}>ID:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{walletInfo.id}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Name:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{walletInfo.name}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Created On:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{walletInfo.createdOn}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Added On:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{walletInfo.addedOn}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Permission:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{walletInfo.permission}</span>
        </p>
      </div>

      {/* DIDs Table */}
      <h3 style={{ color: '#ff00ff', marginBottom: '15px' }}>DIDs</h3>
      <table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          backgroundColor: 'rgba(26, 29, 53, 0.5)',
          border: '2px solid #ff00ff',
          boxShadow: '0 0 20px rgba(255, 0, 255, 0.3)',
        }}
      >
        <thead>
          <tr
            style={{
              borderBottom: '2px solid #ff00ff',
              textAlign: 'left',
              backgroundColor: 'rgba(255, 0, 255, 0.1)',
            }}
          >
            <th
              style={{
                padding: '15px',
                color: '#ff00ff',
                textShadow: '0 0 10px rgba(255, 0, 255, 0.8)',
              }}
            >
              DID
            </th>
            <th
              style={{
                padding: '15px',
                color: '#ff00ff',
                textShadow: '0 0 10px rgba(255, 0, 255, 0.8)',
              }}
            >
              Alias
            </th>
            <th
              style={{
                padding: '15px',
                color: '#ff00ff',
                textShadow: '0 0 10px rgba(255, 0, 255, 0.8)',
              }}
            >
              Key ID
            </th>
            <th
              style={{
                padding: '15px',
                color: '#ff00ff',
                textShadow: '0 0 10px rgba(255, 0, 255, 0.8)',
              }}
            >
              Default
            </th>
            <th
              style={{
                padding: '15px',
                color: '#ff00ff',
                textShadow: '0 0 10px rgba(255, 0, 255, 0.8)',
              }}
            >
              Created On
            </th>
          </tr>
        </thead>
        <tbody>
          {walletInfo.dids.map((did, index) => (
            <tr
              key={index}
              style={{
                borderBottom: '1px solid rgba(255, 0, 255, 0.3)',
                transition: 'all 0.3s ease',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(255, 0, 255, 0.1)';
                e.currentTarget.style.boxShadow = '0 0 15px rgba(255, 0, 255, 0.5)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
                e.currentTarget.style.boxShadow = 'none';
              }}
            >
              <td style={{ padding: '12px', color: '#e0e0e0', wordBreak: 'break-word' }}>
                {did.did}
              </td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{did.alias}</td>
              <td style={{ padding: '12px', color: '#e0e0e0', wordBreak: 'break-word' }}>
                {did.keyId}
              </td>
              <td style={{ padding: '12px' }}>
                <BooleanBadge value={did.default} />
              </td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{did.createdOn}</td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* DID Document Previews */}
      <h3 style={{ color: '#ff00ff', marginTop: '30px', marginBottom: '15px' }}>DID Documents</h3>
      {walletInfo.dids.map((did, index) => (
        <details
          key={index}
          style={{
            border: '2px solid #ff00ff',
            padding: '15px',
            borderRadius: '8px',
            marginBottom: '15px',
            backgroundColor: 'rgba(26, 29, 53, 0.6)',
            boxShadow: '0 0 10px rgba(255, 0, 255, 0.3)',
          }}
        >
          <summary
            style={{
              color: '#ff00ff',
              cursor: 'pointer',
              fontWeight: 'bold',
              marginBottom: '10px',
            }}
          >
            {did.alias} - {did.did}
          </summary>
          <pre
            style={{
              color: '#e0e0e0',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
              margin: 0,
              fontFamily: 'Courier New, monospace',
              fontSize: '0.85em',
              lineHeight: '1.5',
            }}
          >
            {did.document}
          </pre>
        </details>
      ))}
    </div>
  );
};

export default WalletInfo;
