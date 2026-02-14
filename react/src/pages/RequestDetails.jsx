import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { X509 } from 'jsrsasign';
import BooleanBadge from '../components/BooleanBadge';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { ArrowLeft } from 'lucide-react';
import QRCode from 'react-qr-code';

const RequestDetails = () => {
  const { id } = useParams();
  const [request, setRequest] = useState(null);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
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
    setSubmitting(true);
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
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) return <div className="p-8 text-brand-sky">Loading...</div>;
  if (error) return <div className="p-8 text-danger">Error: {error}</div>;
  if (!request) return <div className="p-8 text-muted-foreground">Request not found</div>;

  const showDecisionButtons =
    request.interact_method &&
    request.interact_method.length > 0 &&
    request.interact_method[0] === 'cross-user';

  const getStatusColorClass = (status, isVcIssued) => {
    switch (status?.toLowerCase()) {
      case 'processing':
      case 'proccesing':
        return 'text-yellow-500 border-yellow-500 shadow-yellow-500/30';
      case 'pending':
        return 'text-orange-500 border-orange-500 shadow-orange-500/30';
      case 'approved':
        return 'text-brand-sky border-brand-sky shadow-brand-sky/30';
      case 'finalized':
        return isVcIssued
          ? 'text-green-500 border-green-500 shadow-green-500/30'
          : 'text-red-500 border-red-500 shadow-red-500/30';
      default:
        return 'text-brand-sky border-brand-sky shadow-brand-sky/30';
    }
  };

  const statusClasses = getStatusColorClass(request.status, request.is_vc_issued);

  return (
    <div className="w-full">
      <div className="relative mb-6 flex items-center justify-center">
        <Button
          variant="outline"
          onClick={() => navigate('/requests')}
          className="absolute left-0 border-brand-purple text-brand-purple hover:bg-brand-purple/10 hover:text-brand-purple"
        >
          <ArrowLeft className="mr-2 h-4 w-4" /> Back to List
        </Button>
        <h1 className="text-3xl font-bold text-brand-sky font-ubuntu">Request Details</h1>
      </div>

      <div
        className={cn(
          'rounded-lg border bg-background/60 p-6 shadow-lg text-left mb-6',
          statusClasses,
        )}
      >
        <div className="space-y-4">
          <p>
            <strong className="text-brand-sky">ID:</strong>{' '}
            <span className="text-muted-foreground">{request.id}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Slug:</strong>{' '}
            <span className="text-muted-foreground">{request.participant_slug}</span>
          </p>
          <p>
            <strong className="text-brand-sky">VC Type:</strong>{' '}
            <span className="text-brand-purple">{request.vc_type}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Interact Methods:</strong>{' '}
            <span className="text-muted-foreground">{request.interact_method.join(', ')}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Status:</strong>{' '}
            <span className="font-bold">{request.status}</span>
          </p>
          {request.vc_uri && (
            <div className="space-y-4">
              <div>
                <strong className="text-brand-sky block mb-1">VC URI:</strong>{' '}
                <span className="text-muted-foreground break-all">{request.vc_uri}</span>
              </div>
              <div className="p-4 bg-white/10 rounded-lg inline-block">
                <QRCode
                  value={request.vc_uri}
                  size={150}
                  style={{ height: 'auto', maxWidth: '100%', width: '100%' }}
                  viewBox={`0 0 150 150`}
                />
              </div>
            </div>
          )}
          <p>
            <strong className="text-brand-sky">VC Issued:</strong>{' '}
            <BooleanBadge value={request.is_vc_issued} />
          </p>
          <p>
            <strong className="text-brand-sky">Created At:</strong>{' '}
            <span className="text-muted-foreground">{request.created_at}</span>
          </p>
          {request.ended_at && (
            <p>
              <strong className="text-brand-sky">Ended At:</strong>{' '}
              <span className="text-muted-foreground">{request.ended_at}</span>
            </p>
          )}
        </div>
      </div>

      {request.cert && (
        <div className="rounded-lg border border-brand-purple bg-background/60 p-6 shadow-lg shadow-brand-purple/20 text-left mb-6 text-muted-foreground">
          <h3 className="text-xl font-bold text-brand-purple drop-shadow-md mb-4">
            Certificate Details
          </h3>
          {parsedCert && !parsedCert.error ? (
            <div className="space-y-2">
              <p>
                <strong className="text-brand-purple">Subject:</strong>{' '}
                <span className="break-all inline-block max-w-full">{parsedCert.subject}</span>
              </p>
              <p>
                <strong className="text-brand-purple">Issuer:</strong>{' '}
                <span className="break-all inline-block max-w-full">{parsedCert.issuer}</span>
              </p>
              <p>
                <strong className="text-brand-purple">Serial:</strong>{' '}
                <span>{parsedCert.serial}</span>
              </p>
              <p>
                <strong className="text-brand-purple">Not Before:</strong>{' '}
                <span>{parsedCert.notBefore}</span>
              </p>
              <p>
                <strong className="text-brand-purple">Not After:</strong>{' '}
                <span>{parsedCert.notAfter}</span>
              </p>
            </div>
          ) : (
            <p>
              <em className="text-danger">
                {parsedCert?.error || 'Raw cert available but parsing failed.'}
              </em>
            </p>
          )}
          <details className="mt-4">
            <summary className="text-brand-purple cursor-pointer hover:underline">
              Raw Certificate
            </summary>
            <pre className="mt-2 text-xs overflow-x-auto whitespace-pre-wrap break-all bg-black/40 p-3 rounded border border-brand-purple/50 text-muted-foreground">
              {request.cert}
            </pre>
          </details>
        </div>
      )}

      {showDecisionButtons && request.status === 'Pending' && (
        <div className="flex gap-4 mt-6">
          <Button
            onClick={() => handleDecision(true)}
            disabled={submitting}
            className="bg-green-500/20 text-green-500 border border-green-500 hover:bg-green-500/30 font-bold shadow-lg shadow-green-500/20"
          >
            {submitting ? 'PROCESSING...' : 'APPROVE'}
          </Button>
          <Button
            onClick={() => handleDecision(false)}
            disabled={submitting}
            className="bg-red-500/20 text-red-500 border border-red-500 hover:bg-red-500/30 font-bold shadow-lg shadow-red-500/20"
          >
            {submitting ? 'PROCESSING...' : 'REJECT'}
          </Button>
        </div>
      )}
    </div>
  );
};

export default RequestDetails;
