import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { X509 } from 'jsrsasign';
import BooleanBadge from '../components/BooleanBadge';

const RequestDetails = () => {
  const { id } = useParams();
  const [request, setRequest] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [parsedCert, setParsedCert] = useState(null);
  const navigate = useNavigate();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchRequest = async () => {
      try {
        const response = await fetch(`${apiUrl}/approver/${id}`);
        if (!response.ok) {
          throw new Error('Failed to fetch request details');
        }
        const data = await response.json();
        setRequest(data);

        if (data.cert) {
          try {
            // The user said the cert is "plano" (cleaned). We need to wrap it to parse it as PEM.
            const pem = `-----BEGIN CERTIFICATE-----\n${data.cert}\n-----END CERTIFICATE-----`;
            const x = new X509();
            x.readCertPEM(pem);
            setParsedCert({
              subject: x.getSubjectString(),
              issuer: x.getIssuerString(),
              serial: x.getSerialNumberHex(),
              notBefore: x.getNotBefore(),
              notAfter: x.getNotAfter(),
            });
          } catch (certErr) {
            console.error('Error parsing cert:', certErr);
            setParsedCert({ error: 'Failed to parse certificate' });
          }
        }

        setLoading(false);
      } catch (err) {
        console.error('Error fetching request details:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchRequest();
  }, [id, apiUrl]);

  const handleDecision = async (approve) => {
    try {
      const response = await fetch(`${apiUrl}/approver/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ approve }),
      });

      if (!response.ok) {
        throw new Error('Failed to submit decision');
      }

      // Refresh data or navigate back
      alert(`Request ${approve ? 'Approved' : 'Rejected'} successfully!`);
      navigate('/requests');
    } catch (err) {
      console.error('Error submitting decision:', err);
      alert('Error: ' + err.message);
    }
  };

  if (loading) return <div style={{ padding: '20px', color: '#00f0ff' }}>Loading...</div>;
  if (error) return <div style={{ padding: '20px', color: '#ff0040' }}>Error: {error}</div>;
  if (!request) return <div style={{ padding: '20px', color: '#e0e0e0' }}>Request not found</div>;

  const showDecisionButtons =
    request.interact_method &&
    request.interact_method.length > 0 &&
    request.interact_method[0] === 'cross-user';

  return (
    <div style={{ padding: '30px', minHeight: '100vh' }}>
      <div style={{ position: 'relative', marginBottom: '20px' }}>
        <button
          onClick={() => navigate('/requests')}
          style={{
            position: 'absolute',
            left: 0,
            top: '50%',
            transform: 'translateY(-50%)',
            cursor: 'pointer',
            backgroundColor: 'rgba(189, 0, 255, 0.2)',
            border: '2px solid #bd00ff',
            color: '#bd00ff',
            padding: '8px 16px',
            boxShadow: '0 0 15px rgba(189, 0, 255, 0.4)',
            borderRadius: '4px',
            fontSize: '1em',
            transition: 'all 0.3s ease',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.3)';
            e.currentTarget.style.boxShadow = '0 0 20px rgba(189, 0, 255, 0.6)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.2)';
            e.currentTarget.style.boxShadow = '0 0 15px rgba(189, 0, 255, 0.4)';
          }}
        >
          &larr; Back to List
        </button>
        <h1 style={{ margin: 0, textAlign: 'center' }}>Request Details</h1>
      </div>
      <div
        style={{
          border: '2px solid #00f0ff',
          padding: '25px',
          borderRadius: '8px',
          marginBottom: '20px',
          textAlign: 'left',
          backgroundColor: 'rgba(26, 29, 53, 0.6)',
          boxShadow: '0 0 20px rgba(0, 240, 255, 0.3)',
        }}
      >
        <p>
          <strong style={{ color: '#00f0ff' }}>ID:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{request.id}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Slug:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{request.participant_slug}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>VC Type:</strong>{' '}
          <span style={{ color: '#ff00ff' }}>{request.vc_type}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Interact Methods:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{request.interact_method.join(', ')}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Status:</strong>{' '}
          <span style={{ color: '#bd00ff' }}>{request.status}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>VC Issued:</strong>{' '}
          <BooleanBadge value={request.is_vc_issued} />
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Created At:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{request.created_at}</span>
        </p>
        {request.ended_at && (
          <p>
            <strong style={{ color: '#00f0ff' }}>Ended At:</strong>{' '}
            <span style={{ color: '#e0e0e0' }}>{request.ended_at}</span>
          </p>
        )}
      </div>

      {request.cert && (
        <div
          style={{
            border: '2px solid #ff00ff',
            padding: '25px',
            borderRadius: '8px',
            marginBottom: '20px',
            backgroundColor: 'rgba(26, 29, 53, 0.6)',
            color: '#e0e0e0',
            textAlign: 'left',
            boxShadow: '0 0 20px rgba(255, 0, 255, 0.3)',
          }}
        >
          <h3 style={{ color: '#ff00ff', textShadow: '0 0 10px rgba(255, 0, 255, 0.6)' }}>
            Certificate Details
          </h3>
          {parsedCert && !parsedCert.error ? (
            <>
              <p>
                <strong style={{ color: '#ff00ff' }}>Subject:</strong>{' '}
                <span
                  style={{
                    wordBreak: 'break-word',
                    overflowWrap: 'break-word',
                    display: 'inline-block',
                    maxWidth: '100%',
                  }}
                >
                  {parsedCert.subject}
                </span>
              </p>
              <p>
                <strong style={{ color: '#ff00ff' }}>Issuer:</strong>{' '}
                <span
                  style={{
                    wordBreak: 'break-word',
                    overflowWrap: 'break-word',
                    display: 'inline-block',
                    maxWidth: '100%',
                  }}
                >
                  {parsedCert.issuer}
                </span>
              </p>
              <p>
                <strong style={{ color: '#ff00ff' }}>Serial:</strong>{' '}
                <span>{parsedCert.serial}</span>
              </p>
              <p>
                <strong style={{ color: '#ff00ff' }}>Not Before:</strong>{' '}
                <span>{parsedCert.notBefore}</span>
              </p>
              <p>
                <strong style={{ color: '#ff00ff' }}>Not After:</strong>{' '}
                <span>{parsedCert.notAfter}</span>
              </p>
            </>
          ) : (
            <p>
              <em style={{ color: '#ff0040' }}>
                {parsedCert?.error || 'Raw cert available but parsing failed.'}
              </em>
            </p>
          )}
          <details>
            <summary style={{ color: '#ff00ff', cursor: 'pointer', marginTop: '10px' }}>
              Raw Certificate
            </summary>
            <pre
              style={{
                overflowX: 'auto',
                whiteSpace: 'pre-wrap',
                wordBreak: 'break-all',
                backgroundColor: 'rgba(10, 14, 39, 0.8)',
                padding: '10px',
                borderRadius: '4px',
                border: '1px solid #ff00ff',
                color: '#e0e0e0',
                marginTop: '10px',
              }}
            >
              {request.cert}
            </pre>
          </details>
        </div>
      )}

      {showDecisionButtons && request.status !== 'completed' && request.status !== 'rejected' && (
        <div style={{ display: 'flex', gap: '20px', marginTop: '20px' }}>
          <button
            onClick={() => handleDecision(true)}
            style={{
              padding: '12px 24px',
              backgroundColor: 'rgba(0, 255, 65, 0.2)',
              color: '#00ff41',
              border: '2px solid #00ff41',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '16px',
              fontWeight: 'bold',
              boxShadow: '0 0 20px rgba(0, 255, 65, 0.4)',
              transition: 'all 0.3s ease',
            }}
            onMouseEnter={(e) => {
              e.target.style.backgroundColor = 'rgba(0, 255, 65, 0.3)';
              e.target.style.boxShadow = '0 0 30px rgba(0, 255, 65, 0.6)';
            }}
            onMouseLeave={(e) => {
              e.target.style.backgroundColor = 'rgba(0, 255, 65, 0.2)';
              e.target.style.boxShadow = '0 0 20px rgba(0, 255, 65, 0.4)';
            }}
          >
            Approve
          </button>
          <button
            onClick={() => handleDecision(false)}
            style={{
              padding: '12px 24px',
              backgroundColor: 'rgba(255, 0, 64, 0.2)',
              color: '#ff0040',
              border: '2px solid #ff0040',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '16px',
              fontWeight: 'bold',
              boxShadow: '0 0 20px rgba(255, 0, 64, 0.4)',
              transition: 'all 0.3s ease',
            }}
            onMouseEnter={(e) => {
              e.target.style.backgroundColor = 'rgba(255, 0, 64, 0.3)';
              e.target.style.boxShadow = '0 0 30px rgba(255, 0, 64, 0.6)';
            }}
            onMouseLeave={(e) => {
              e.target.style.backgroundColor = 'rgba(255, 0, 64, 0.2)';
              e.target.style.boxShadow = '0 0 20px rgba(255, 0, 64, 0.4)';
            }}
          >
            Reject
          </button>
        </div>
      )}
    </div>
  );
};

export default RequestDetails;
