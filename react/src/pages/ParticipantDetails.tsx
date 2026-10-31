import React, { useMemo } from "react";
import { useNavigate, useParams } from "@tanstack/react-router";
import {
  Activity,
  ArrowLeft,
  Calendar,
  Cpu,
  Globe,
  Key,
  Shield,
} from "lucide-react";
import {
  Badge,
  type BadgeRole,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  decodePathId,
  encodePathId,
  FormatDate,
  GeneralErrorComponent,
  Identifier,
  InfoList,
  type InfoItemProps,
  JsonTable,
  PageHeader,
  PageLayout,
  Separator,
  Skeleton,
} from "@eunomia/ui";
import {
  useGetMyself,
  useGetParticipantById,
} from "@/data/orval/participants/participants";

export default function ParticipantDetails() {
  const { participantId: rawParam } = useParams({
    from: "/participants/$participantId/",
  });
  const navigate = useNavigate();

  const participantId = useMemo(() => {
    if (!rawParam) return "";
    try {
      return decodePathId(rawParam);
    } catch {
      return decodeURIComponent(rawParam);
    }
  }, [rawParam]);

  const { data: res, isLoading, isError, error, refetch } =
    // The API takes the DID as base64url too, so its `:` and `%3A` survive the path.
    useGetParticipantById(encodePathId(participantId), {
      query: { enabled: Boolean(participantId) },
    });

  const { data: myselfRes } = useGetMyself({
    query: { retry: false },
  });

  if (isLoading) {
    return (
      <PageLayout>
        <PageHeader title="Participant Details" />
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
        : new Error("Failed to fetch participant details");
    return <GeneralErrorComponent error={finalError} reset={() => void refetch()} />;
  }

  const participant = res.data;
  const isMe =
    myselfRes?.status === 200 &&
    myselfRes.data.participant_id === participant.participant_id;

  const hasExtraFields =
    participant.extra_fields &&
    typeof participant.extra_fields === "object" &&
    Object.keys(participant.extra_fields).length > 0;

  const infoItems: InfoItemProps[] = [
    {
      label: "Nickname",
      value: participant.participant_nick || "Unnamed Participant",
    },
    {
      label: "Identifier (DID)",
      value: {
        type: "custom",
        content: <Identifier value={participant.participant_id} />,
      },
    },
    {
      label: "Participant Role",
      value: {
        type: "custom",
        content: (
          <Badge
            variant="role"
            dsrole={participant.participant_type as BadgeRole}
          >
            {participant.participant_type}
          </Badge>
        ),
      },
    },
    ...(participant.base_url
      ? [
          {
            label: "Base URL",
            value: {
              type: "custom" as const,
              content: (
                <a
                  href={participant.base_url}
                  target="_blank"
                  rel="noreferrer"
                  className="inline-flex items-center gap-1.5 text-brand-sky hover:underline font-mono text-xs"
                >
                  <Globe className="h-3.5 w-3.5" />
                  {participant.base_url}
                </a>
              ),
            },
          },
        ]
      : []),
  ];

  return (
    <PageLayout>
      <PageHeader
        title={participant.participant_nick || "Participant Details"}
        badge={
          <div className="flex items-center gap-2">
            <Badge
              variant="role"
              dsrole={participant.participant_type as BadgeRole}
            >
              {participant.participant_type}
            </Badge>
            {isMe && <Badge variant="info">Local Authority</Badge>}
          </div>
        }
      >
        <Button
          variant="link"
          className="mt-1 px-0"
          onClick={() => void navigate({ to: "/participants" })}
        >
          <ArrowLeft className="mr-1 h-4 w-4" />
          Back to participants
        </Button>
      </PageHeader>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <Shield className="h-4 w-4 text-brand-sky" />
                Identity Information
              </CardTitle>
            </CardHeader>
            <CardContent>
              <InfoList items={infoItems} />
            </CardContent>
          </Card>

          {hasExtraFields && (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-base font-semibold flex items-center gap-2">
                  <Cpu className="h-4 w-4 text-brand-sky" />
                  Extended Metadata
                </CardTitle>
              </CardHeader>
              <CardContent>
                <JsonTable value={participant.extra_fields} />
              </CardContent>
            </Card>
          )}
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <Calendar className="h-4 w-4 text-brand-sky" />
                Activity Timestamps
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between items-center text-sm gap-4">
                <span className="text-muted-foreground flex items-center gap-2">
                  <Key className="h-3.5 w-3.5" />
                  First Registered
                </span>
                <span className="font-medium">
                  <FormatDate date={participant.saved_at} />
                </span>
              </div>
              <Separator />
              <div className="flex justify-between items-center text-sm gap-4">
                <span className="text-muted-foreground flex items-center gap-2">
                  <Activity className="h-3.5 w-3.5" />
                  Last Interaction
                </span>
                <span className="font-medium">
                  {participant.last_interaction ? (
                    <FormatDate date={participant.last_interaction} />
                  ) : (
                    "—"
                  )}
                </span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </PageLayout>
  );
}
