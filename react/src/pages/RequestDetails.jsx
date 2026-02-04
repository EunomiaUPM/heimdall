import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { X509 } from 'jsrsasign';

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

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!request) return <div>Request not found</div>;

  const showDecisionButtons =
    request.interact_method &&
    request.interact_method.length > 0 &&
    request.interact_method[0] === 'cross-user';

  return (
    <div style={{ padding: '20px' }}>
      <button
        onClick={() => navigate('/requests')}
        style={{ marginBottom: '20px', cursor: 'pointer' }}
      >
        &larr; Back to List
      </button>
      <h1>Request Details</h1>
      <div
        style={{
          border: '1px solid #ccc',
          padding: '20px',
          borderRadius: '8px',
          marginBottom: '20px',
        }}
      >
        <p>
          <strong>ID:</strong> {request.id}
        </p>
        <p>
          <strong>Slug:</strong> {request.participant_slug}
        </p>
        <p>
          <strong>VC Type:</strong> {request.vc_type}
        </p>
        <p>
          <strong>Status:</strong> {request.status}
        </p>
        <p>
          <strong>Created At:</strong> {request.created_at}
        </p>
        <p>
          <strong>Interact Methods:</strong> {request.interact_method.join(', ')}
        </p>
        {request.vc_uri && (
          <p>
            <strong>VC URI:</strong> {request.vc_uri}
          </p>
        )}
        {request.vc_issuing && (
          <p>
            <strong>VC Issuing:</strong> {request.vc_issuing}
          </p>
        )}
        {request.ended_at && (
          <p>
            <strong>Ended At:</strong> {request.ended_at}
          </p>
        )}
        <p>
          <strong>VC Issued:</strong> {request.is_vc_issued ? 'Yes' : 'No'}
        </p>
      </div>

      {request.cert && (
        <div
          style={{
            border: '1px solid #ccc',
            padding: '20px',
            borderRadius: '8px',
            marginBottom: '20px',
            backgroundColor: '#f9f9f9',
          }}
        >
          <h3>Certificate Details</h3>
          {parsedCert && !parsedCert.error ? (
            <>
              <p>
                <strong>Subject:</strong> {parsedCert.subject}
              </p>
              <p>
                <strong>Issuer:</strong> {parsedCert.issuer}
              </p>
              <p>
                <strong>Serial:</strong> {parsedCert.serial}
              </p>
              <p>
                <strong>Not Before:</strong> {parsedCert.notBefore}
              </p>
              <p>
                <strong>Not After:</strong> {parsedCert.notAfter}
              </p>
            </>
          ) : (
            <p>
              <em>{parsedCert?.error || 'Raw cert available but parsing failed.'}</em>
            </p>
          )}
          <details>
            <summary>Raw Certificate</summary>
            <pre style={{ overflowX: 'auto', whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
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
              padding: '10px 20px',
              backgroundColor: 'green',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '16px',
            }}
          >
            Approve
          </button>
          <button
            onClick={() => handleDecision(false)}
            style={{
              padding: '10px 20px',
              backgroundColor: 'red',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
              fontSize: '16px',
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
