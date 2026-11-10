import { createFileRoute } from "@tanstack/react-router";
import IssuedDetails from "@/pages/IssuedDetails";

export const Route = createFileRoute("/issued/$issuedId/")({
  component: IssuedDetails,
});
