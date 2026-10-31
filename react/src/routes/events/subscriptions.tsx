import { createFileRoute } from "@tanstack/react-router";
import { EventSubscriptions } from "@eunomia/ui/events";
import { EventTopics } from "@/lib/eventTopics";

export const Route = createFileRoute("/events/subscriptions")({
  component: () => <EventSubscriptions topics={EventTopics} defaultPattern="heimdall:**" />,
});
