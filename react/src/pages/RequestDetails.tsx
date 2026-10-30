import React, { useMemo, useState } from "react";
import { useNavigate, useParams } from "@tanstack/react-router";
import { useQueryClient } from "@tanstack/react-query";
import { X509 } from "jsrsasign";
import { toast } from "sonner";
import {
  ArrowLeft,
  CheckCircle2,
  Eye,
  EyeOff,
  FileBadge2,
  FileJson,
  Handshake,
  KeyRound,
  ShieldAlert,
  ShieldCheck,
  XCircle,
} from "lucide-react";
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ConfirmDialog,
  FormatDate,
  GeneralErrorComponent,
  getFriendlyVCType,
  Identifier,
  InfoList,
  type InfoItemProps,
  JsonTable,
  PageHeader,
  PageLayout,
  ParticipantLabel,
  Skeleton,
} from "@eunomia/ui";
import {
  getGetAllRequestsQueryKey,
  getGetRequestDetailsQueryKey,
  useGetRequestDetails,
  useManageRequest,
} from "@/data/orval/approver/approver";

interface ParsedCertInfo {
  subject: string;
  issuer: string;
  serial: string;
  notBefore: string;
  notAfter: string;
  sigAlg: string;
}

function parseX509Pem(pem: string): ParsedCertInfo | null {
  try {
    const normalized = pem.includes("BEGIN CERTIFICATE")
      ? pem
      : `-----BEGIN CERTIFICATE-----\n${pem.trim()}\n-----END CERTIFICATE-----`;
    const cert = new X509();
    cert.readCertPEM(normalized);
    return {
      subject: cert.getSubjectString(),
      issuer: cert.getIssuerString(),
      serial: cert.getSerialNumberHex(),
      notBefore: cert.getNotBefore(),
      notAfter: cert.getNotAfter(),
      sigAlg: cert.getSignatureAlgorithmField(),
    };
  } catch {
    return null;
  }
}

function SecretField({ value }: { value?: string | null }) {
  const [revealed, setRevealed] = useState(false);

  if (!value) {
    return <span className="text-muted-foreground">—</span>;
  }

  return (
    <div className="inline-flex items-center gap-2 max-w-full">
      <code className="font-mono text-xs bg-background-200/60 px-2 py-0.5 rounded break-all">
        {revealed ? value : "••••••••••••••••"}
      </code>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        className="h-6 w-6 shrink-0"
        onClick={() => setRevealed((prev) => !prev)}
        title={revealed ? "Hide value" : "Reveal value"}
      >
        {revealed ? (
          <EyeOff className="h-3.5 w-3.5" />
        ) : (
          <Eye className="h-3.5 w-3.5" />
        )}
      </Button>
    </div>
  );
}

export default function RequestDetails() {
  const { requestId: rawId } = useParams({
    from: "/requests/$requestId/",
  });
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const requestId = useMemo(
    () => (rawId ? decodeURIComponent(rawId) : ""),
    [rawId],
  );

  const [rejectOpen, setRejectOpen] = useState(false);

  const {
    data: res,
    isLoading,
    isError,
    error,
    refetch,
  } = useGetRequestDetails(requestId, {
    query: { enabled: Boolean(requestId) },
  });

  const manageMutation = useManageRequest();

  const certInfo = useMemo(() => {
    const certPem = res?.status === 200 ? res.data.issuance?.build_ctx?.cert : null;
    if (!certPem) return null;
    return parseX509Pem(certPem);
  }, [res]);

  const submitDecision = async (approve: boolean) => {
    const response = await manageMutation.mutateAsync({
      id: requestId,
      data: { approve },
    });

    if (response.status !== 200) {
      const msg = `Failed to ${approve ? "approve" : "reject"} request (HTTP ${response.status})`;
      toast.error(msg);
      throw new Error(msg);
    }

    toast.success(
      approve
        ? "Credential request approved"
        : "Credential request rejected",
    );

    await Promise.all([
      queryClient.invalidateQueries({
        queryKey: getGetRequestDetailsQueryKey(requestId),
      }),
      queryClient.invalidateQueries({
        queryKey: getGetAllRequestsQueryKey(),
      }),
    ]);
  };

  const handleApprove = async () => {
    try {
      await submitDecision(true);
    } catch {
      // Toast already displayed in submitDecision
    }
  };

  if (isLoading) {
    return (
      <PageLayout>
        <PageHeader title="VC Request Details" />
        <div className="space-y-6 w-full">
          <Skeleton className="h-48 w-full rounded-xl" />
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Skeleton className="h-64 w-full rounded-xl" />
            <Skeleton className="h-64 w-full rounded-xl" />
          </div>
        </div>
      </PageLayout>
    );
  }

  if (isError || !res || res.status !== 200) {
    const finalError =
      error instanceof Error
        ? error
        : new Error("Failed to fetch request details");
    return (
      <GeneralErrorComponent error={finalError} reset={() => void refetch()} />
    );
  }

  const { grant, issuance, interaction, verification } = res.data;
  const isPending = grant.status === "Pending";

  const grantItems: InfoItemProps[] = [
    {
      label: "Grant ID",
      value: {
        type: "custom",
        content: <Identifier value={grant.id} />,
      },
    },
    {
      label: "Participant",
      value: {
        type: "custom",
        content: grant.participant_id ? (
          <ParticipantLabel
            id={grant.participant_id}
            nick={grant.participant_nick}
          />
        ) : (
          <span className="font-medium">
            {grant.participant_nick || "Anonymous"}
          </span>
        ),
      },
    },
    {
      label: "Status",
      value: {
        type: "custom",
        content: (
          <Badge variant="status" state={grant.status}>
            {grant.status}
          </Badge>
        ),
      },
    },
    {
      label: "Grant Kind",
      value: grant.kind,
    },
    {
      label: "Requested Role",
      value: {
        type: "custom",
        content: (
          <Badge variant="outline" className="font-mono text-xs">
            {grant.role}
          </Badge>
        ),
      },
    },
    {
      label: "Visibility",
      value: {
        type: "custom",
        content: <Badge variant="infoLighter">{grant.visibility}</Badge>,
      },
    },
    {
      label: "Credential Types",
      value: {
        type: "custom",
        content:
          grant.vc_type_config && grant.vc_type_config.length > 0 ? (
            <div className="flex flex-wrap gap-1.5">
              {grant.vc_type_config.map((vcType) => (
                <Badge key={vcType} variant="info">
                  {getFriendlyVCType(vcType)}
                </Badge>
              ))}
            </div>
          ) : (
            <span className="text-muted-foreground">—</span>
          ),
      },
    },
    {
      label: "Issuance State",
      value: {
        type: "custom",
        content: issuance?.credential ? (
          <Badge variant="status" state="completed">
            Issued
          </Badge>
        ) : (
          <Badge variant="status" state="pending">
            Pending Issuance
          </Badge>
        ),
      },
    },
    {
      label: "Created At",
      value: {
        type: "custom",
        content: <FormatDate date={grant.created_at} />,
      },
    },
    ...(grant.ended_at
      ? [
          {
            label: "Ended At",
            value: {
              type: "custom" as const,
              content: <FormatDate date={grant.ended_at} />,
            },
          },
        ]
      : []),
  ];

  const issuanceItems: InfoItemProps[] = issuance
    ? [
        {
          label: "Issuance ID",
          value: {
            type: "custom",
            content: <Identifier value={issuance.id} />,
          },
        },
        {
          label: "Subject Name",
          value: issuance.subject_name,
        },
        {
          label: "Holder DID",
          value: issuance.build_ctx.holder_did
            ? {
                type: "custom",
                content: <Identifier value={issuance.build_ctx.holder_did} />,
              }
            : "—",
        },
        {
          label: "Issuer DID",
          value: {
            type: "custom",
            content: <Identifier value={issuance.issuer_did} />,
          },
        },
        {
          label: "Credential ID",
          value: {
            type: "custom",
            content: <Identifier value={issuance.credential_id} />,
          },
        },
        {
          label: "Audience",
          value: issuance.aud,
        },
        {
          label: "Pre-Authorized Code",
          value: {
            type: "custom",
            content: <SecretField value={issuance.pre_auth_code} />,
          },
        },
        {
          label: "Access Token",
          value: {
            type: "custom",
            content: <SecretField value={issuance.token} />,
          },
        },
        {
          label: "Nonce",
          value: {
            type: "custom",
            content: <SecretField value={issuance.nonce} />,
          },
        },
      ]
    : [];

  const interactionItems: InfoItemProps[] = interaction
    ? [
        {
          label: "Interaction ID",
          value: {
            type: "custom",
            content: <Identifier value={interaction.id} />,
          },
        },
        {
          label: "Finish Method",
          value: interaction.method,
        },
        {
          label: "Callback URI",
          value: {
            type: "custom",
            content: (
              <span className="font-mono text-xs break-all">
                {interaction.callback_uri}
              </span>
            ),
          },
        },
        {
          label: "Continuation Endpoint",
          value: {
            type: "custom",
            content: (
              <span className="font-mono text-xs break-all">
                {interaction.continuation_endpoint}
              </span>
            ),
          },
        },
        {
          label: "Continuation Wait",
          value:
            interaction.continuation_wait != null
              ? `${interaction.continuation_wait}s`
              : "—",
        },
        {
          label: "Interact Ref",
          value: {
            type: "custom",
            content: <SecretField value={interaction.interact_ref} />,
          },
        },
        {
          label: "AS Nonce",
          value: {
            type: "custom",
            content: <SecretField value={interaction.as_nonce} />,
          },
        },
        {
          label: "Hash Method",
          value: interaction.hash_method,
        },
      ]
    : [];

  const verificationItems: InfoItemProps[] = verification
    ? [
        {
          label: "Verification ID",
          value: {
            type: "custom",
            content: <Identifier value={verification.id} />,
          },
        },
        {
          label: "Status",
          value: {
            type: "custom",
            content: (
              <Badge variant="status" state={verification.status}>
                {verification.status}
              </Badge>
            ),
          },
        },
        {
          label: "Holder",
          value: verification.holder
            ? {
                type: "custom",
                content: <Identifier value={verification.holder} />,
              }
            : "—",
        },
        {
          label: "Audience",
          value: verification.audience,
        },
        {
          label: "State",
          value: {
            type: "custom",
            content: <SecretField value={verification.state} />,
          },
        },
        {
          label: "Nonce",
          value: {
            type: "custom",
            content: <SecretField value={verification.nonce} />,
          },
        },
        {
          label: "Presented VCs",
          value: `${verification.vcs.length} credential(s)`,
        },
        {
          label: "Created At",
          value: {
            type: "custom",
            content: <FormatDate date={verification.created_at} />,
          },
        },
      ]
    : [];

  return (
    <PageLayout>
      <PageHeader
        title={grant.participant_nick || "VC Request Details"}
        badge={
          <Badge variant="status" state={grant.status}>
            {grant.status}
          </Badge>
        }
      >
        <Button
          variant="link"
          className="mt-1 px-0"
          onClick={() => void navigate({ to: "/requests" })}
        >
          <ArrowLeft className="mr-1 h-4 w-4" />
          Back to requests
        </Button>
      </PageHeader>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <KeyRound className="h-4 w-4 text-brand-sky" />
                Grant Overview
              </CardTitle>
            </CardHeader>
            <CardContent>
              <InfoList items={grantItems} />
            </CardContent>
          </Card>

          {issuance && (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base font-semibold flex items-center gap-2">
                  <FileBadge2 className="h-4 w-4 text-brand-sky" />
                  Credential Issuance (OID4VCI)
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-6">
                <InfoList items={issuanceItems} />

                {issuance.build_ctx.cert && (
                  <div className="rounded-lg border border-border bg-background-200/20 p-4 space-y-3">
                    <p className="text-xs font-semibold uppercase tracking-wider text-brand-sky">
                      X.509 Certificate Evidence
                    </p>
                    {certInfo ? (
                      <InfoList
                        items={[
                          { label: "Subject", value: certInfo.subject },
                          { label: "Issuer", value: certInfo.issuer },
                          { label: "Serial (Hex)", value: certInfo.serial },
                          { label: "Signature Alg", value: certInfo.sigAlg },
                          { label: "Valid From", value: certInfo.notBefore },
                          { label: "Valid Until", value: certInfo.notAfter },
                        ]}
                      />
                    ) : (
                      <pre className="font-mono text-xs overflow-x-auto whitespace-pre-wrap break-all text-muted-foreground">
                        {issuance.build_ctx.cert}
                      </pre>
                    )}
                  </div>
                )}

                {issuance.build_ctx.claims &&
                  Object.keys(issuance.build_ctx.claims).length > 0 && (
                    <div className="space-y-2">
                      <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                        Requested Claims
                      </p>
                      <JsonTable value={issuance.build_ctx.claims} />
                    </div>
                  )}
              </CardContent>
            </Card>
          )}

          {interaction && (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base font-semibold flex items-center gap-2">
                  <Handshake className="h-4 w-4 text-brand-sky" />
                  GNAP Interaction
                </CardTitle>
              </CardHeader>
              <CardContent>
                <InfoList items={interactionItems} />
              </CardContent>
            </Card>
          )}

          {verification && (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base font-semibold flex items-center gap-2">
                  <ShieldCheck className="h-4 w-4 text-brand-sky" />
                  Presentation Verification (OID4VP)
                </CardTitle>
              </CardHeader>
              <CardContent>
                <InfoList items={verificationItems} />
              </CardContent>
            </Card>
          )}

          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <FileJson className="h-4 w-4 text-brand-sky" />
                Raw Request Payload
              </CardTitle>
            </CardHeader>
            <CardContent>
              <JsonTable value={res.data} />
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <ShieldAlert className="h-4 w-4 text-brand-sky" />
                Authority Decision
              </CardTitle>
              <CardDescription>
                {isPending
                  ? "Review the peer identity and cryptographic evidence before issuing this credential."
                  : `This request is currently ${grant.status.toLowerCase()} and requires no further operator action.`}
              </CardDescription>
            </CardHeader>
            <CardContent>
              {isPending ? (
                <div className="flex flex-col gap-3">
                  <Button
                    className="w-full"
                    onClick={() => void handleApprove()}
                    isLoading={manageMutation.isPending}
                  >
                    <CheckCircle2 className="mr-2 h-4 w-4" />
                    Approve Request
                  </Button>
                  <Button
                    variant="destructive"
                    className="w-full"
                    disabled={manageMutation.isPending}
                    onClick={() => setRejectOpen(true)}
                  >
                    <XCircle className="mr-2 h-4 w-4" />
                    Reject Request
                  </Button>
                </div>
              ) : (
                <div className="rounded-lg border border-border bg-background-200/20 px-4 py-3 text-sm text-muted-foreground">
                  Decision recorded:{" "}
                  <span className="font-semibold text-foreground">
                    {grant.status}
                  </span>
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </div>

      <ConfirmDialog
        open={rejectOpen}
        onOpenChange={setRejectOpen}
        title="Reject credential request?"
        description="Rejecting this request terminates the GNAP grant and prevents credential issuance for this peer."
        confirmLabel="Reject request"
        destructive
        onConfirm={() => submitDecision(false)}
      />
    </PageLayout>
  );
}
