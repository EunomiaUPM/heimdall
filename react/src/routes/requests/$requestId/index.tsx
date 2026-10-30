import { createFileRoute } from "@tanstack/react-router";
import RequestDetails from "@/pages/RequestDetails";

export const Route = createFileRoute("/requests/$requestId/")({
  component: RequestDetails,
});
