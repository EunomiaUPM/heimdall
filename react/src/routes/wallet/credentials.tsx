import { createFileRoute } from "@tanstack/react-router";
import { WalletCredentialsView } from "@eunomia/ui";

const HeimdallWalletCredentials = () => (
  <WalletCredentialsView
    showGaiaCompliance={false}
    requestVcHref={() => "/requests"}
  />
);

export const Route = createFileRoute("/wallet/credentials")({
  component: HeimdallWalletCredentials,
});
