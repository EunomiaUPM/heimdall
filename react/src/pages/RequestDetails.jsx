import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { X509 } from 'jsrsasign';
import { ArrowLeft } from 'lucide-react';
import QRCode from 'react-qr-code';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { formatIdentifier } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';
import { InfoGrid } from '@/components/layout/InfoGrid';
import { InfoList } from '@/components/ui/info-list';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';

const getIdentityProofDisplay = (methods) => {
  if (!methods || methods.length === 0) return '—';
  if (methods.length === 1 && methods[0] === '') return 'Certificate';
  if (methods.includes('oidc4vp')) return 'Verifiable Credential';
  return methods.join(', ');
};

const RequestDetails = () => {
  const { id } = useParams();
  const [request, setRequest] = useState(null);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState(null);
  const [parsedCert, setParsedCert] = useState(null);
  const navigate = useNavigate();

  const fetchRequest = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/approver/${id}`);
      if (!response.ok) throw new Error('Failed to fetch request details');
      const data = await response.json();
      setRequest(data);

      if (data.cert) {
        try {
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
    } catch (err) {
      console.error('Error fetching request details:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchRequest();
  }, [id]);

  const handleDecision = async (approve) => {
    setSubmitting(true);
    try {
      const response = await fetch(`${apiUrl}/approver/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approve }),
      });
      if (!response.ok) throw new Error('Failed to submit decision');
      navigate('/requests');
    } catch (err) {
      console.error('Error submitting decision:', err);
      alert('Error: ' + err.message);
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) {
    return (
      <PageLayout>
        <PageHeader title="Request Details" />
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchRequest} />;

  if (!request) {
    return (
      <PageLayout>
        <PageHeader title="Request Details" />
        <p className="text-muted-foreground italic">Request not found.</p>
      </PageLayout>
    );
  }

  const isCertificateAuth =
    request.interact_method &&
    request.interact_method.length === 1 &&
    request.interact_method[0] === '';

  const showDecisionButtons = isCertificateAuth && request.status === 'Pending';

  return (
    <PageLayout>
      <PageHeader
        title="Request Details"
        badge={
          <Badge variant="info" size="lg">
            {formatIdentifier(request.id)}
          </Badge>
        }
      >
        <Button
          variant="link"
          className="mt-2 px-0"
          onClick={() => navigate('/requests')}
        >
          <ArrowLeft className="mr-1 h-4 w-4" /> Back to requests
        </Button>
      </PageHeader>

      <InfoGrid>
        <PageSection title="Request">
          <InfoList
            items={[
              { label: 'Request ID', value: { type: 'urn', value: request.id } },
              { label: 'Alias', value: request.participant_slug || '—' },
              { label: 'VC Type', value: request.vc_type },
              {
                label: 'Identity Proof',
                value: getIdentityProofDisplay(request.interact_method),
              },
              {
                label: 'Status',
                value: { type: 'status', value: request.status },
              },
              {
                label: 'VC Issued',
                value: {
                  type: 'custom',
                  content: request.is_vc_issued ? (
                    <Badge variant="status" state="success">
                      Issued
                    </Badge>
                  ) : (
                    <Badge variant="status" state="warn">
                      Pending
                    </Badge>
                  ),
                },
              },
            ]}
          />
        </PageSection>

        <PageSection title="Activity">
          <InfoList
            items={[
              {
                label: 'Created at',
                value: { type: 'date', value: request.created_at },
              },
              request.ended_at
                ? { label: 'Ended at', value: { type: 'date', value: request.ended_at } }
                : null,
            ].filter(Boolean)}
          />
        </PageSection>
      </InfoGrid>

      {request.vc_uri && (
        <PageSection title="Verifiable Credential URI">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 items-start">
            <div className="md:col-span-2 space-y-2">
              <p className="text-[10px] uppercase tracking-wide text-white/50 font-medium">
                URI
              </p>
              <div className="p-3 rounded-md bg-background-200/30 border border-white/10 break-all font-mono text-xs text-white/80">
                {request.vc_uri}
              </div>
            </div>
            <div className="p-4 bg-white/95 rounded-lg w-fit">
              <QRCode
                value={request.vc_uri}
                size={150}
                style={{ height: 'auto', maxWidth: '100%', width: '100%' }}
                viewBox="0 0 150 150"
              />
            </div>
          </div>
        </PageSection>
      )}

      {request.cert && (
        <PageSection title="Certificate Details">
          {parsedCert && !parsedCert.error ? (
            <InfoList
              items={[
                { label: 'Subject', value: parsedCert.subject },
                { label: 'Issuer', value: parsedCert.issuer },
                { label: 'Serial', value: parsedCert.serial },
                { label: 'Not Before', value: parsedCert.notBefore },
                { label: 'Not After', value: parsedCert.notAfter },
              ]}
            />
          ) : (
            <p className="text-sm text-destructive italic">
              {parsedCert?.error || 'Raw cert available but parsing failed.'}
            </p>
          )}
          <details className="mt-4 group">
            <summary className="text-xs uppercase tracking-wide text-white/60 cursor-pointer hover:text-white/80">
              Raw Certificate
            </summary>
            <pre className="mt-2 text-xs overflow-x-auto whitespace-pre-wrap break-all bg-black/40 p-3 rounded border border-white/10 text-muted-foreground">
              {request.cert}
            </pre>
          </details>
        </PageSection>
      )}

      {showDecisionButtons && (
        <PageSection title="Decision">
          <div className="flex gap-3">
            <Button
              onClick={() => handleDecision(true)}
              disabled={submitting}
              className="bg-success-600/20 text-success-300 border border-success-600 hover:bg-success-600/30 font-semibold"
            >
              {submitting ? 'Processing…' : 'Approve'}
            </Button>
            <Button
              onClick={() => handleDecision(false)}
              disabled={submitting}
              className="bg-danger-600/20 text-danger-300 border border-danger-600 hover:bg-danger-600/30 font-semibold"
            >
              {submitting ? 'Processing…' : 'Reject'}
            </Button>
          </div>
        </PageSection>
      )}
    </PageLayout>
  );
};

export default RequestDetails;
