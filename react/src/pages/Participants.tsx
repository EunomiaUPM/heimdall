import React from "react";
import { Link, useNavigate } from "react-router-dom";
import { ArrowRight } from "lucide-react";
import {
  Badge,
  type BadgeRole,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  cn,
  DataTable,
  encodePathId,
  FormatDate,
  GeneralErrorComponent,
  Identifier,
  PageHeader,
  PageLayout,
  PageSection,
  Skeleton,
} from "@eunomia/ui";
import {
  useGetAllParticipants,
  useGetMyself,
} from "@/data/orval/participants/participants";
import type { ParticipantDto } from "@/data/orval/model";

export default function Participants() {
  const navigate = useNavigate();

  const {
    data: participantsRes,
    isLoading,
    isFetching,
    isError,
    error,
    refetch,
  } = useGetAllParticipants();

  const { data: myselfRes } = useGetMyself({
    query: { retry: false },
  });

  const myAgent = myselfRes?.status === 200 ? myselfRes.data : undefined;
  const participants: ParticipantDto[] =
    participantsRes?.status === 200 ? participantsRes.data : [];

  if (isLoading && !participantsRes) {
    return (
      <PageLayout>
        <PageHeader title="Participants" />
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8 w-full">
          {[1, 2, 3].map((i) => (
            <Skeleton key={i} className="h-32 w-full rounded-xl" />
          ))}
        </div>
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (isError || (participantsRes && participantsRes.status !== 200)) {
    const finalError =
      error instanceof Error ? error : new Error("Failed to load participants");
    return <GeneralErrorComponent error={finalError} reset={() => void refetch()} />;
  }

  return (
    <PageLayout>
      <PageHeader
        title="Participants"
        description="This authority node and the dataspace agents and peers it has interacted with."
      />

      {myAgent && (
        <div className="mb-2 w-full">
          <Card className="bg-background-200/15 overflow-hidden relative">
            <CardHeader className="pb-2">
              <div className="flex justify-between items-center gap-4">
                <div>
                  <p className="text-xs font-semibold text-brand-sky uppercase tracking-wider mb-1">
                    My Local Authority
                  </p>
                  <CardTitle className="text-2xl">
                    {myAgent.participant_nick || "Authority Node"}
                  </CardTitle>
                </div>
                <Badge variant="status" state="active">
                  Active
                </Badge>
              </div>
            </CardHeader>
            <CardContent>
              <div className="flex justify-between items-end flex-wrap gap-6 mt-2">
                <div className="flex flex-wrap gap-8">
                  <div className="space-y-1">
                    <p className="text-xs text-muted-foreground uppercase">
                      DID Identifier
                    </p>
                    <Identifier value={myAgent.participant_id} />
                  </div>
                  <div className="space-y-1">
                    <p className="text-xs text-muted-foreground uppercase">
                      System Role
                    </p>
                    <div>
                      <Badge
                        variant="role"
                        dsrole={myAgent.participant_type as BadgeRole}
                      >
                        {myAgent.participant_type}
                      </Badge>
                    </div>
                  </div>
                  <div className="space-y-1">
                    <p className="text-xs text-muted-foreground uppercase">
                      Base URL
                    </p>
                    <Badge variant="info">{myAgent.base_url}</Badge>
                  </div>
                </div>

                <Link to={`/participants/${encodePathId(myAgent.participant_id)}`}>
                  <Button variant="link" className="px-0">
                    View My Profile
                    <ArrowRight className="ml-1 h-4 w-4" />
                  </Button>
                </Link>
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      <PageSection title="Network Participants">
        <DataTable<ParticipantDto>
          className="text-sm"
          data={participants}
          loading={isFetching}
          keyExtractor={(p) => p.participant_id}
          onRowClick={(p) =>
            navigate(`/participants/${encodePathId(p.participant_id)}`)
          }
          emptyMessage="No network participants registered yet"
          searchPlaceholder="Filter participants by name, DID, or URL..."
          filters={[
            {
              id: "type",
              label: "Type",
              accessorKey: "participant_type",
              options: [
                { label: "Agent", value: "Agent" },
                { label: "Authority", value: "Authority" },
              ],
            },
          ]}
          columns={[
            {
              header: "Participant",
              accessorKey: "participant_nick",
              cell: (p) => {
                const isMe =
                  Boolean(myAgent) && p.participant_id === myAgent?.participant_id;
                return (
                  <div className="flex items-center gap-3">
                    <div
                      className={cn(
                        "w-8 h-8 rounded-lg flex items-center justify-center font-bold text-xs shrink-0",
                        isMe
                          ? "bg-brand-sky/20 text-brand-sky border border-brand-sky/30"
                          : "bg-background-200 text-muted-foreground",
                      )}
                    >
                      {(p.participant_nick || "U").charAt(0).toUpperCase()}
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="font-medium">
                        {p.participant_nick || "Unknown"}
                      </span>
                      {isMe && (
                        <span className="rounded bg-brand-sky/15 px-1.5 py-px text-[0.6875rem] font-semibold text-brand-sky">
                          You
                        </span>
                      )}
                    </div>
                  </div>
                );
              },
            },
            {
              header: "DID Identifier",
              accessorKey: "participant_id",
              cell: (p) => <Identifier value={p.participant_id} />,
            },
            {
              header: "Role",
              accessorKey: "participant_type",
              cell: (p) => (
                <Badge variant="role" dsrole={p.participant_type as BadgeRole}>
                  {p.participant_type}
                </Badge>
              ),
            },
            {
              header: "Base URL",
              accessorKey: "base_url",
              cell: (p) => (
                <span className="font-mono text-xs text-muted-foreground">
                  {p.base_url || "—"}
                </span>
              ),
            },
            {
              header: "Added on",
              accessorKey: "saved_at",
              sortKey: "created_at",
              sortValue: (p) => p.saved_at,
              cell: (p) => (
                <span className="text-muted-foreground">
                  <FormatDate date={p.saved_at} />
                </span>
              ),
            },
            {
              header: "Actions",
              searchable: false,
              cell: (p) => (
                <Button
                  variant="link"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    navigate(`/participants/${encodePathId(p.participant_id)}`);
                  }}
                >
                  See agent
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
