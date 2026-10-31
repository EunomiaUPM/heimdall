import { createFileRoute, Outlet } from "@tanstack/react-router";
import { EventsLayout } from "@eunomia/ui/events";

export const Route = createFileRoute("/events")({
  component: () => (
    <EventsLayout
      basePath="/events"
      description="Events heimdall publishes on its bus about the records it creates, edits and deletes."
    >
      <Outlet />
    </EventsLayout>
  ),
});
