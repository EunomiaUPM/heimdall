import { createFileRoute } from "@tanstack/react-router";
import Participants from "@/pages/Participants";

export const Route = createFileRoute("/participants/")({
  component: Participants,
});
