import { createFileRoute } from "@tanstack/react-router";
import { pathIdParams } from "@eunomia/ui";
import ParticipantDetails from "@/pages/ParticipantDetails";

export const Route = createFileRoute("/participants/$participantId/")({
  params: pathIdParams("participantId"),
  component: ParticipantDetails,
});
