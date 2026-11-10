import React from "react";
import { useNavigate } from "@tanstack/react-router";
import { ArrowRight } from "lucide-react";
import {
  Badge,
  Button,
  DataTable,
  FormatDate,
  GeneralErrorComponent,
  getFriendlyVCType,
  Identifier,
  PageHeader,
  PageLayout,
  PageSection,
  Skeleton,
} from "@eunomia/ui";
import { useGetAllIssued } from "@/data/orval/issued/issued";
import type { IssuedVcDto } from "@/data/orval/model";

export type IssuedState = "Active" | "Suspended" | "Revoked";

export function issuedState(vc: IssuedVcDto): IssuedState {
  if (vc.revoked_at) return "Revoked";
  if (vc.suspended_at) return "Suspended";
  return "Active";
}

export function IssuedStateBadge({ vc }: { vc: IssuedVcDto }) {
  const state = issuedState(vc);
  const variant =
    state === "Revoked" ? "destructive" : state === "Suspended" ? "warning" : "success";
  return <Badge variant={variant}>{state}</Badge>;
}

export default function Issued() {
  const navigate = useNavigate();

  const {
    data: issuedRes,
    isLoading,
    isFetching,
    isError,
    error,
    refetch,
  } = useGetAllIssued();

  const issued: IssuedVcDto[] = issuedRes?.status === 200 ? issuedRes.data : [];

  if (isLoading && !issuedRes) {
    return (
      <PageLayout>
        <PageHeader title="Issued VCs" />
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (isError || (issuedRes && issuedRes.status !== 200)) {
    const finalError =
      error instanceof Error ? error : new Error("Failed to fetch issued credentials");
    return <GeneralErrorComponent error={finalError} reset={() => void refetch()} />;
  }

  const openDetails = (vc: IssuedVcDto) =>
    void navigate({
      to: "/issued/$issuedId",
      params: { issuedId: vc.issuance_id },
    });

  return (
    <PageLayout>
      <PageHeader
        title="Issued VCs"
        description="Credentials issued by this authority and their revocation and suspension status."
      />

      <PageSection title="Issued Credentials">
        <DataTable<IssuedVcDto>
          className="text-sm"
          data={issued}
          loading={isFetching}
          keyExtractor={(vc) => vc.credential_id}
          onRowClick={openDetails}
          emptyMessage="No issued credentials found"
          emptyDescription="Credentials appear here once a holder retrieves them through OID4VCI."
          searchPlaceholder="Filter by subject, holder DID, type or ID..."
          columns={[
            {
              header: "Subject",
              accessorKey: "subject_name",
              cell: (vc) => <span className="font-medium">{vc.subject_name}</span>,
            },
            {
              header: "Credential Type",
              accessorKey: "vc_type",
              cell: (vc) => (
                <div className="flex flex-wrap gap-1.5">
                  {vc.vc_type.split(", ").map((t) => (
                    <Badge key={t} variant="info">
                      {getFriendlyVCType(t)}
                    </Badge>
                  ))}
                </div>
              ),
            },
            {
              header: "Holder",
              accessorKey: "holder_did",
              cell: (vc) => <Identifier value={vc.holder_did} />,
            },
            {
              header: "Status",
              accessorKey: "revoked_at",
              searchable: false,
              cell: (vc) => <IssuedStateBadge vc={vc} />,
            },
            {
              header: "Issued",
              accessorKey: "issued_at",
              sortKey: "issued_at",
              sortValue: (vc) => vc.issued_at,
              cell: (vc) => (
                <span className="text-muted-foreground">
                  <FormatDate date={vc.issued_at} />
                </span>
              ),
            },
            {
              header: "Actions",
              searchable: false,
              cell: (vc) => (
                <Button
                  variant="link"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    openDetails(vc);
                  }}
                >
                  See credential
                  <ArrowRight className="ml-1 h-3.5 w-3.5" />
                </Button>
              ),
            },
          ]}
        />
      </PageSection>
    </PageLayout>
  );
}
