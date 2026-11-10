import { createFileRoute } from "@tanstack/react-router";
import Issued from "@/pages/Issued";

export const Route = createFileRoute("/issued/")({
  component: Issued,
});
