import { useMemo, useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { X509 } from 'jsrsasign';
import {
  ArrowLeft,
  Award,
  CheckCircle2,
  Eye,
  EyeOff,
  FileJson,
  Inbox,
  Key,
} from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { formatIdentifier, getFriendlyVCType, cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';
import { FormatDate } from '@/components/ui/format-date';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';

const RequestDetails = () => {
  const { id } = useParams();
  const [details, setDetails] = useState(null);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  const fetchDetails = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/approver/${id}/details`);
      if (!response.ok) throw new Error('Failed to fetch request details');
      const data = await response.json();
      setDetails(data);
    } catch (err) {
      console.error('Error fetching request details:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDetails();
  }, [id]);

  const parsedCert = useMemo(() => {
    const cert = details?.issuance?.build_ctx?.cert;
    if (!cert) return null;
    try {
      const pem = `-----BEGIN CERTIFICATE-----\n${cert}\n-----END CERTIFICATE-----`;
      const x = new X509();
      x.readCertPEM(pem);
      return {
        subject: x.getSubjectString(),
        issuer: x.getIssuerString(),
        serial: x.getSerialNumberHex(),
        notBefore: x.getNotBefore(),
        notAfter: x.getNotAfter(),
      };
    } catch (e) {
      return { error: 'Failed to parse certificate' };
    }
  }, [details]);

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

  if (error) return <GeneralErrorComponent error={error} reset={fetchDetails} />;

  const grant = details?.grant ?? null;
  const issuance = details?.issuance ?? null;
  const interaction = details?.interaction ?? null;
  const verification = details?.verification ?? null;

  if (!grant) {
    return (
      <PageLayout>
        <PageHeader title="Request Details" />
        <p className="text-muted-foreground italic">Request not found.</p>
      </PageLayout>
    );
  }

  const showDecisionButtons = grant.status === 'Pending';

  return (
    <PageLayout>
      <PageHeader
        title="Request Details"
        badge={
          <Badge variant="info" size="lg">
            {formatIdentifier(grant.id)}
          </Badge>
        }
      >
        <Button variant="link" className="mt-2 px-0" onClick={() => navigate('/requests')}>
          <ArrowLeft className="mr-1 h-4 w-4" /> Back to requests
        </Button>
      </PageHeader>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* ===== Grant ============================================================= */}
        <Card className="md:col-span-2">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Inbox className="h-5 w-5 text-primary" />
              Grant
            </CardTitle>
            <CardDescription>The credential request received from a peer.</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-6 gap-x-4">
              <DetailItem label="Status">
                <Badge variant="status" state={grant.status}>
                  {grant.status || '-'}
                </Badge>
              </DetailItem>
              <DetailItem label="Kind">
                <Badge variant="info" className="font-mono">
                  {grant.kind}
                </Badge>
              </DetailItem>
              <DetailItem label="Peer Nick">{grant.participant_nick || '-'}</DetailItem>
              <DetailItem label="Issued Token">
                <SecretField value={grant.token} />
              </DetailItem>
              <DetailItem label="VC Types Requested">
                {(grant.vc_type_config ?? []).length === 0 ? (
                  <span className="text-xs text-muted-foreground">—</span>
                ) : (
                  <div className="flex flex-wrap gap-1">
                    {grant.vc_type_config.map((cfg, idx) => (
                      <Badge key={idx} variant="role">
                        {getFriendlyVCType(cfg)}
                      </Badge>
                    ))}
                  </div>
                )}
              </DetailItem>
              <DetailItem label="VC Issued">
                {grant.token ? (
                  <Badge variant="status" state="success">
                    Issued
                  </Badge>
                ) : (
                  <Badge variant="status" state="warn">
                    Pending
                  </Badge>
                )}
              </DetailItem>
              <DetailItem label="Received At">
                <FormatDate date={grant.created_at} />
              </DetailItem>
              <DetailItem label="Ended At">
                {grant.ended_at ? <FormatDate date={grant.ended_at} /> : '—'}
              </DetailItem>
            </div>
          </CardContent>
        </Card>

        {/* ===== Decision =========================================================== */}
        <Card>
          <CardHeader>
            <CardTitle>Decision</CardTitle>
            <CardDescription>
              {showDecisionButtons
                ? 'Approve or reject this credential request.'
                : 'This request is no longer pending.'}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {showDecisionButtons ? (
              <div className="flex flex-col gap-3">
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
            ) : (
              <div className="text-sm text-muted-foreground italic">
                Current state: <Badge variant="status" state={grant.status}>{grant.status}</Badge>
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* ===== Issuance =========================================================== */}
      {issuance ? (
        <Card className="mt-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Award className="h-5 w-5 text-success-500" />
              Issuance
            </CardTitle>
            <CardDescription>The credential being prepared and (eventually) emitted.</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-6 gap-x-4">
              <DetailItem label="Subject Name">{issuance.subject_name || '—'}</DetailItem>
              <DetailItem label="Issuer DID">
                <span className="font-mono text-[10px] break-all">{issuance.issuer_did}</span>
              </DetailItem>
              <DetailItem label="Audience">
                <span className="font-mono text-[10px] break-all">{issuance.aud}</span>
              </DetailItem>
              <DetailItem label="Credential ID">
                <span className="font-mono text-[10px] break-all">{issuance.credential_id}</span>
              </DetailItem>
              <DetailItem label="Holder DID">
                {issuance.build_ctx?.holder_did ? (
                  <span className="font-mono text-[10px] break-all">
                    {issuance.build_ctx.holder_did}
                  </span>
                ) : (
                  <span className="text-xs text-muted-foreground">—</span>
                )}
              </DetailItem>
              <DetailItem label="VC Types">
                {(issuance.vc_type_config ?? []).length === 0 ? (
                  <span className="text-xs text-muted-foreground">—</span>
                ) : (
                  <div className="flex flex-wrap gap-1">
                    {issuance.vc_type_config.map((cfg, idx) => (
                      <Badge key={idx} variant="role">
                        {getFriendlyVCType(cfg)}
                      </Badge>
                    ))}
                  </div>
                )}
              </DetailItem>
              <DetailItem label="Pre-Auth Code">
                <SecretField value={issuance.pre_auth_code} />
              </DetailItem>
              <DetailItem label="Token">
                <SecretField value={issuance.token} />
              </DetailItem>
              <DetailItem label="Token Expiration (s)">
                <span className="font-mono text-[10px]">
                  {issuance.token_expiration ?? '—'}
                </span>
              </DetailItem>
              <DetailItem label="Nonce">
                <SecretField value={issuance.nonce} />
              </DetailItem>
              <DetailItem label="Signed Credential (JWT)">
                <SecretField value={issuance.credential} />
              </DetailItem>
            </div>

            {parsedCert && (
              <div className="pt-4 border-t border-stroke">
                <h5 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">
                  Holder Certificate
                </h5>
                {parsedCert.error ? (
                  <p className="text-sm text-destructive italic">{parsedCert.error}</p>
                ) : (
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-4 gap-x-4">
                    <DetailItem label="Subject">
                      <span className="font-mono text-[10px] break-all">{parsedCert.subject}</span>
                    </DetailItem>
                    <DetailItem label="Issuer">
                      <span className="font-mono text-[10px] break-all">{parsedCert.issuer}</span>
                    </DetailItem>
                    <DetailItem label="Serial">
                      <span className="font-mono text-[10px] break-all">{parsedCert.serial}</span>
                    </DetailItem>
                    <DetailItem label="Not Before">
                      <span className="font-mono text-[10px]">{parsedCert.notBefore}</span>
                    </DetailItem>
                    <DetailItem label="Not After">
                      <span className="font-mono text-[10px]">{parsedCert.notAfter}</span>
                    </DetailItem>
                  </div>
                )}
                <details className="mt-4 group">
                  <summary className="text-xs uppercase tracking-wide text-white/60 cursor-pointer hover:text-white/80">
                    Raw Certificate
                  </summary>
                  <pre className="mt-2 text-xs overflow-x-auto whitespace-pre-wrap break-all bg-black/40 p-3 rounded border border-white/10 text-muted-foreground">
                    {issuance.build_ctx?.cert}
                  </pre>
                </details>
              </div>
            )}
          </CardContent>
        </Card>
      ) : (
        <Card className="mt-6 opacity-50 border-dashed">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-muted-foreground">
              <Award className="h-5 w-5" />
              Issuance
            </CardTitle>
            <CardDescription>No issuance record bound to this grant yet.</CardDescription>
          </CardHeader>
        </Card>
      )}

      {/* ===== Interaction ========================================================= */}
      {interaction ? (
        <Card className="mt-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Key className="h-5 w-5 text-primary" />
              Interaction
            </CardTitle>
            <CardDescription>GNAP interaction handshake initiated by the peer.</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-6 gap-x-4">
              <DetailItem label="Method">
                <Badge variant="info" className="font-mono">
                  {String(interaction.method)}
                </Badge>
              </DetailItem>
              <DetailItem label="Start">
                <div className="flex flex-wrap gap-1">
                  {(interaction.start ?? []).map((s, idx) => (
                    <Badge key={idx} variant="role">
                      {typeof s === 'string' ? s : Object.keys(s ?? {})[0] ?? '?'}
                    </Badge>
                  ))}
                </div>
              </DetailItem>
              <DetailItem label="Callback URI">
                <span className="font-mono text-[10px] break-all">{interaction.callback_uri}</span>
              </DetailItem>
              <DetailItem label="Continue Endpoint">
                <span className="font-mono text-[10px] break-all">
                  {interaction.continue_endpoint || '—'}
                </span>
              </DetailItem>
              <DetailItem label="Hash Method">
                <span className="font-mono text-[10px]">
                  {typeof interaction.hash_method === 'string'
                    ? interaction.hash_method
                    : Object.keys(interaction.hash_method ?? {})[0] ?? '—'}
                </span>
              </DetailItem>
              <DetailItem label="Continue Wait">
                <span className="font-mono text-[10px]">{interaction.continue_wait ?? '—'}</span>
              </DetailItem>
              <DetailItem label="Interact Ref">
                <SecretField value={interaction.interact_ref} />
              </DetailItem>
              <DetailItem label="AS Nonce">
                <SecretField value={interaction.as_nonce} />
              </DetailItem>
            </div>
          </CardContent>
        </Card>
      ) : (
        <Card className="mt-6 opacity-50 border-dashed">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-muted-foreground">
              <Key className="h-5 w-5" />
              Interaction
            </CardTitle>
            <CardDescription>
              Not required — this grant was approved directly without a GNAP handshake.
            </CardDescription>
          </CardHeader>
        </Card>
      )}

      {/* ===== Verification ======================================================== */}
      {verification ? (
        <Card className="mt-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <CheckCircle2 className="h-5 w-5 text-amber-500" />
              Verification (OID4VP)
            </CardTitle>
            <CardDescription>
              We requested a presentation from the peer before issuing the credential.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-y-6 gap-x-4">
              <DetailItem label="Status">
                <Badge variant="status" state={verification.status}>
                  {verification.status}
                </Badge>
              </DetailItem>
              <DetailItem label="State">
                <SecretField value={verification.state} />
              </DetailItem>
              <DetailItem label="Nonce">
                <SecretField value={verification.nonce} />
              </DetailItem>
              <DetailItem label="Audience">
                <span className="font-mono text-[10px] break-all">{verification.audience}</span>
              </DetailItem>
              <DetailItem label="Holder">
                {verification.holder ? (
                  <span className="font-mono text-[10px] break-all">{verification.holder}</span>
                ) : (
                  <span className="text-xs text-muted-foreground">—</span>
                )}
              </DetailItem>
              <DetailItem label="VC Types">
                {(verification.vc_type ?? []).length === 0 ? (
                  <span className="text-xs text-muted-foreground">—</span>
                ) : (
                  <div className="flex flex-wrap gap-1">
                    {verification.vc_type.map((t, idx) => (
                      <Badge key={idx} variant="role">
                        {typeof t === 'string' ? t : Object.keys(t ?? {})[0] ?? '?'}
                      </Badge>
                    ))}
                  </div>
                )}
              </DetailItem>
              <DetailItem label="Presented VCs count">
                <span className="font-mono text-xs">{verification.vcs?.length ?? 0}</span>
              </DetailItem>
              <DetailItem label="Created At">
                <FormatDate date={verification.created_at} />
              </DetailItem>
              <DetailItem label="Ended At">
                {verification.ended_at ? <FormatDate date={verification.ended_at} /> : '—'}
              </DetailItem>
            </div>
          </CardContent>
        </Card>
      ) : interaction ? (
        <Card className="mt-6 opacity-50 border-dashed">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-muted-foreground">
              <CheckCircle2 className="h-5 w-5" />
              Verification (OID4VP)
            </CardTitle>
            <CardDescription>
              Not required — we didn't ask the peer for a presentation.
            </CardDescription>
          </CardHeader>
        </Card>
      ) : null}

      <RawDetails details={details} />
    </PageLayout>
  );
};

const DetailItem = ({ label, children, labelClassName }) => (
  <div className="flex flex-col gap-1.5">
    <span
      className={cn(
        'text-xs font-semibold uppercase tracking-wider',
        labelClassName || 'text-muted-foreground',
      )}
    >
      {label}
    </span>
    <div className="text-sm font-medium">{children}</div>
  </div>
);

const SecretField = ({ value }) => {
  const [revealed, setRevealed] = useState(false);
  if (!value) return <span className="font-mono text-[10px] text-muted-foreground">—</span>;
  return (
    <div className="flex items-center gap-2">
      <span className="font-mono text-[10px] break-all flex-1 select-all">
        {revealed ? value : '•'.repeat(Math.min(value.length, 24))}
      </span>
      <button
        type="button"
        onClick={() => setRevealed((v) => !v)}
        className="text-muted-foreground hover:text-foreground transition-colors p-1 rounded hover:bg-white/5"
        aria-label={revealed ? 'Hide value' : 'Reveal value'}
        title={revealed ? 'Hide' : 'Reveal'}
      >
        {revealed ? <EyeOff className="h-3 w-3" /> : <Eye className="h-3 w-3" />}
      </button>
    </div>
  );
};

const RawDetails = ({ details }) => {
  const [open, setOpen] = useState(false);
  if (!details) return null;
  return (
    <div className="mt-6 border border-white/10 rounded-xl overflow-hidden bg-white/[0.02]">
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between p-3 text-xs font-mono uppercase tracking-widest text-muted-foreground hover:bg-white/[0.04] transition-colors"
      >
        <span className="flex items-center gap-2">
          <FileJson className="h-3 w-3" />
          Raw JSON
        </span>
        <span>{open ? '−' : '+'}</span>
      </button>
      {open && (
        <pre className="p-4 bg-black/40 font-mono text-[11px] text-muted-foreground/90 whitespace-pre-wrap break-all leading-relaxed overflow-x-auto">
          {JSON.stringify(details, null, 2)}
        </pre>
      )}
    </div>
  );
};

export default RequestDetails;
