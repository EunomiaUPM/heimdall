import React from "react";
import { useNavigate } from "react-router-dom";
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
  ParticipantLabel,
  Skeleton,
} from "@eunomia/ui";
import { useGetAllRequests } from "@/data/orval/approver/approver";
import type { RecvGrantDto } from "@/data/orval/model";

export default function Requests() {
  const navigate = useNavigate();

  const {
    data: requestsRes,
    isLoading,
    isFetching,
    isError,
    error,
    refetch,
  } = useGetAllRequests();

  const requests: RecvGrantDto[] =
    requestsRes?.status === 200 ? requestsRes.data : [];

  if (isLoading && !requestsRes) {
    return (
      <PageLayout>
        <PageHeader title="VC Requests" />
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (isError || (requestsRes && requestsRes.status !== 200)) {
    const finalError =
      error instanceof Error
        ? error
        : new Error("Failed to fetch credential requests");
    return (
      <GeneralErrorComponent error={finalError} reset={() => void refetch()} />
    );
  }

  return (
    <PageLayout>
      <PageHeader
        title="VC Requests"
        description="Verifiable credential requests submitted by peers to this authority."
      />

      <PageSection title="Incoming Requests">
        <DataTable<RecvGrantDto>
          className="text-sm"
          data={requests}
          loading={isFetching}
          keyExtractor={(r) => r.id}
          onRowClick={(r) => navigate(`/requests/${encodeURIComponent(r.id)}`)}
          emptyMessage="No credential requests found"
          emptyDescription="Incoming GNAP / OID4VCI requests from dataspace peers will appear here."
          searchPlaceholder="Filter requests by participant, DID, type, or ID..."
          filters={[
            {
              id: "status",
              label: "Status",
              accessorKey: "status",
              options: [
                { label: "Pending", value: "Pending" },
                { label: "Processing", value: "Processing" },
                { label: "Approved", value: "Approved" },
                { label: "Rejected", value: "Rejected" },
                { label: "Finalized", value: "Finalized" },
              ],
            },
          ]}
          columns={[
            {
              header: "Participant",
              accessorKey: "participant_nick",
              cell: (r) =>
                r.participant_id ? (
                  <ParticipantLabel
                    id={r.participant_id}
                    nick={r.participant_nick}
                  />
                ) : (
                  <span className="font-medium">
                    {r.participant_nick || "Anonymous"}
                  </span>
                ),
            },
            {
              header: "Request ID",
              accessorKey: "id",
              cell: (r) => <Identifier value={r.id} />,
            },
            {
              header: "Credential Types",
              accessorKey: "vc_type_config",
              cell: (r) => {
                const types = r.vc_type_config ?? [];
                if (types.length === 0) {
                  return <span className="text-muted-foreground">—</span>;
                }
                return (
                  <div className="flex flex-wrap gap-1.5">
                    {types.map((t) => (
                      <Badge key={t} variant="info">
                        {getFriendlyVCType(t)}
                      </Badge>
                    ))}
                  </div>
                );
              },
            },
            {
              header: "Role / Visibility",
              accessorKey: "role",
              cell: (r) => (
                <div className="flex flex-wrap items-center gap-1.5">
                  <Badge variant="outline" className="font-mono text-xs">
                    {r.role}
                  </Badge>
                  <Badge variant="infoLighter">{r.visibility}</Badge>
                </div>
              ),
            },
            {
              header: "Status",
              accessorKey: "status",
              cell: (r) => (
                <Badge variant="status" state={r.status}>
                  {r.status}
                </Badge>
              ),
            },
            {
              header: "Created",
              accessorKey: "created_at",
              sortKey: "created_at",
              sortValue: (r) => r.created_at,
              cell: (r) => (
                <span className="text-muted-foreground">
                  <FormatDate date={r.created_at} />
                </span>
              ),
            },
            {
              header: "Actions",
              searchable: false,
              cell: (r) => (
                <Button
                  variant="link"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    navigate(`/requests/${encodeURIComponent(r.id)}`);
                  }}
                >
                  See request
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
