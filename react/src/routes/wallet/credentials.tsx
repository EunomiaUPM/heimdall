import { createFileRoute } from "@tanstack/react-router";
import { WalletCredentials } from "@/components/credentials/WalletCredentials";

const HeimdallWalletCredentials = () => (
  <WalletCredentials
    showGaiaCompliance={false}
    requestVcHref={() => "/requests"}
  />
);

export const Route = createFileRoute("/wallet/credentials")({
  component: HeimdallWalletCredentials,
});
