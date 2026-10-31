import { createFileRoute } from "@tanstack/react-router";
import { EventFeed } from "@eunomia/ui/events";
import { EventTopics } from "@/lib/eventTopics";

export const Route = createFileRoute("/events/feed")({
  component: () => <EventFeed topics={EventTopics} />,
});
