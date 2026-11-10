import React, { useMemo, useState } from "react";
import { useNavigate, useParams } from "@tanstack/react-router";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import {
  ArrowLeft,
  Ban,
  FileBadge2,
  FileJson,
  PauseCircle,
  PlayCircle,
  ShieldAlert,
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
  Skeleton,
} from "@eunomia/ui";
import {
  getGetAllIssuedQueryKey,
  getGetIssuedByIdQueryKey,
  useGetIssuedById,
  useReinstateIssued,
  useRevokeIssued,
  useSuspendIssued,
} from "@/data/orval/issued/issued";
import { IssuedStateBadge, issuedState } from "@/pages/Issued";

function decodeJwtPayload(jwt?: string | null): unknown {
  if (!jwt) return null;
  try {
    const payload = jwt.split(".")[1];
    const base64 = payload.replace(/-/g, "+").replace(/_/g, "/");
    const padded = base64 + "=".repeat((4 - (base64.length % 4)) % 4);
    const bytes = Uint8Array.from(atob(padded), (c) => c.charCodeAt(0));
    return JSON.parse(new TextDecoder().decode(bytes));
  } catch {
    return null;
  }
}

type Change = "revoke" | "suspend" | "reinstate";

const CHANGE_DONE: Record<Change, string> = {
  revoke: "Credential revoked",
  suspend: "Credential suspended",
  reinstate: "Credential reinstated",
};

export default function IssuedDetails() {
  const { issuedId: rawId } = useParams({ from: "/issued/$issuedId/" });
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const issuedId = useMemo(() => (rawId ? decodeURIComponent(rawId) : ""), [rawId]);

  const [revokeOpen, setRevokeOpen] = useState(false);

  const {
    data: res,
    isLoading,
    isError,
    error,
    refetch,
  } = useGetIssuedById(issuedId, { query: { enabled: Boolean(issuedId) } });

  const revokeMutation = useRevokeIssued();
  const suspendMutation = useSuspendIssued();
  const reinstateMutation = useReinstateIssued();
  const busy =
    revokeMutation.isPending || suspendMutation.isPending || reinstateMutation.isPending;

  const vcPayload = useMemo(
    () => decodeJwtPayload(res?.status === 200 ? res.data.credential : null),
    [res],
  );

  const applyChange = async (change: Change) => {
    const mutation =
      change === "revoke"
        ? revokeMutation
        : change === "suspend"
          ? suspendMutation
          : reinstateMutation;
    const response = await mutation.mutateAsync({ id: issuedId });
    if (response.status !== 200) {
      const msg = `Failed to ${change} credential (HTTP ${response.status})`;
      toast.error(msg);
      throw new Error(msg);
    }
    toast.success(CHANGE_DONE[change]);
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: getGetIssuedByIdQueryKey(issuedId) }),
      queryClient.invalidateQueries({ queryKey: getGetAllIssuedQueryKey() }),
    ]);
  };

  const runChange = async (change: Change) => {
    try {
      await applyChange(change);
    } catch {
      return;
    }
  };

  if (isLoading) {
    return (
      <PageLayout>
        <PageHeader title="Issued VC Details" />
        <div className="space-y-6 w-full">
          <Skeleton className="h-48 w-full rounded-xl" />
          <Skeleton className="h-64 w-full rounded-xl" />
        </div>
      </PageLayout>
    );
  }

  if (isError || !res || res.status !== 200) {
    const finalError =
      error instanceof Error ? error : new Error("Failed to fetch the issued credential");
    return <GeneralErrorComponent error={finalError} reset={() => void refetch()} />;
  }

  const vc = res.data;
  const state = issuedState(vc);

  const items: InfoItemProps[] = [
    {
      label: "Credential ID",
      value: { type: "custom", content: <Identifier value={vc.credential_id} /> },
    },
    {
      label: "Request ID",
      value: { type: "custom", content: <Identifier value={vc.issuance_id} /> },
    },
    { label: "Subject Name", value: vc.subject_name },
    {
      label: "Holder DID",
      value: { type: "custom", content: <Identifier value={vc.holder_did} /> },
    },
    {
      label: "Credential Type",
      value: {
        type: "custom",
        content: (
          <div className="flex flex-wrap gap-1.5">
            {vc.vc_type.split(", ").map((t) => (
              <Badge key={t} variant="info">
                {getFriendlyVCType(t)}
              </Badge>
            ))}
          </div>
        ),
      },
    },
    {
      label: "Status",
      value: { type: "custom", content: <IssuedStateBadge vc={vc} /> },
    },
    {
      label: "Status List",
      value: `List ${vc.list_no}, index ${vc.idx}`,
    },
    {
      label: "Issued At",
      value: { type: "custom", content: <FormatDate date={vc.issued_at} /> },
    },
    ...(vc.suspended_at
      ? [
          {
            label: "Suspended At",
            value: { type: "custom" as const, content: <FormatDate date={vc.suspended_at} /> },
          },
        ]
      : []),
    ...(vc.revoked_at
      ? [
          {
            label: "Revoked At",
            value: { type: "custom" as const, content: <FormatDate date={vc.revoked_at} /> },
          },
        ]
      : []),
  ];

  return (
    <PageLayout>
      <PageHeader
        title="Issued VC Details"
        description={vc.subject_name}
        actions={
          <Button variant="outline" onClick={() => void navigate({ to: "/issued" })}>
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back
          </Button>
        }
      />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <FileBadge2 className="h-4 w-4 text-brand-sky" />
                Credential
              </CardTitle>
            </CardHeader>
            <CardContent>
              <InfoList items={items} />
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <FileJson className="h-4 w-4 text-brand-sky" />
                Signed Credential (JWT payload)
              </CardTitle>
            </CardHeader>
            <CardContent>
              {vcPayload ? (
                <JsonTable value={vcPayload} />
              ) : (
                <span className="text-muted-foreground">—</span>
              )}
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <ShieldAlert className="h-4 w-4 text-brand-sky" />
                Credential Status
              </CardTitle>
              <CardDescription>
                {state === "Revoked"
                  ? "This credential is revoked. Revocation cannot be undone."
                  : "Changes are published in the status lists and apply to new presentations."}
              </CardDescription>
            </CardHeader>
            <CardContent>
              {state === "Revoked" ? (
                <div className="rounded-lg border border-border bg-background-200/20 px-4 py-3 text-sm text-muted-foreground">
                  Revoked <FormatDate date={vc.revoked_at!} />
                </div>
              ) : (
                <div className="flex flex-col gap-3">
                  {state === "Suspended" ? (
                    <Button
                      className="w-full"
                      onClick={() => void runChange("reinstate")}
                      isLoading={reinstateMutation.isPending}
                      disabled={busy}
                    >
                      <PlayCircle className="mr-2 h-4 w-4" />
                      Reinstate
                    </Button>
                  ) : (
                    <Button
                      variant="outline"
                      className="w-full"
                      onClick={() => void runChange("suspend")}
                      isLoading={suspendMutation.isPending}
                      disabled={busy}
                    >
                      <PauseCircle className="mr-2 h-4 w-4" />
                      Suspend
                    </Button>
                  )}
                  <Button
                    variant="destructive"
                    className="w-full"
                    disabled={busy}
                    onClick={() => setRevokeOpen(true)}
                  >
                    <Ban className="mr-2 h-4 w-4" />
                    Revoke
                  </Button>
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </div>

      <ConfirmDialog
        open={revokeOpen}
        onOpenChange={setRevokeOpen}
        title="Revoke credential?"
        description="Revocation is permanent: verifiers will reject this credential in every new presentation."
        confirmLabel="Revoke credential"
        destructive
        onConfirm={() => applyChange("revoke")}
      />
    </PageLayout>
  );
}
