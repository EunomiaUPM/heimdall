import { createFileRoute } from "@tanstack/react-router";
import { DeadLetters } from "@eunomia/ui/events";

export const Route = createFileRoute("/events/dlq")({
  component: DeadLetters,
});
