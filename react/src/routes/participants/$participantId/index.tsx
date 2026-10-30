import { createFileRoute } from "@tanstack/react-router";
import ParticipantDetails from "@/pages/ParticipantDetails";

export const Route = createFileRoute("/participants/$participantId/")({
  component: ParticipantDetails,
});
