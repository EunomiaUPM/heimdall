import { createFileRoute, Outlet } from "@tanstack/react-router";
import { WalletLayout } from "@eunomia/ui";

const WalletRouteLayout = () => (
  <WalletLayout basePath="/wallet">
    <Outlet />
  </WalletLayout>
);

export const Route = createFileRoute("/wallet")({
  component: WalletRouteLayout,
});
