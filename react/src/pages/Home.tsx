import React, { useMemo } from "react";
import {
  ClipboardCheck,
  Clock,
  FileCheck2,
  KeyRound,
  ShieldCheck,
  Users,
  Wallet,
} from "lucide-react";
import {
  FeatureCard,
  PageHeader,
  PageLayout,
  PageSection,
  StatTile,
} from "@eunomia/ui";
import { useGetAllRequests } from "@/data/orval/approver/approver";
import { useGetAllParticipants } from "@/data/orval/participants/participants";
import { useGetWalletCredentials } from "@/data/orval/wallet/wallet";

export default function Home() {
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE !== "false";

  const { data: requestsRes, isLoading: loadingRequests } = useGetAllRequests();
  const { data: participantsRes, isLoading: loadingParticipants } =
    useGetAllParticipants();
  const { data: vcsRes, isLoading: loadingVcs } = useGetWalletCredentials({
    query: { enabled: walletActive, retry: false },
  });

  const requests = requestsRes?.status === 200 ? requestsRes.data : [];
  const participants =
    participantsRes?.status === 200 ? participantsRes.data : [];
  const vcs = vcsRes?.status === 200 ? vcsRes.data : [];

  const pendingCount = useMemo(
    () => requests.filter((r) => r.status === "Pending").length,
    [requests],
  );

  const features = [
    {
      icon: ClipboardCheck,
      title: "Approver",
      description:
        "Review and decide on incoming Verifiable Credential requests with X.509 and OID4VP evidence inspection.",
      to: "/requests",
    },
    {
      icon: Users,
      title: "Network Participants",
      description:
        "Inspect registered dataspace agents, authorities, and DID profiles interacting with this authority.",
      to: "/participants",
    },
    {
      icon: KeyRound,
      title: "GateKeeper (GNAP)",
      description:
        "Controls fine-grained grant negotiations and continuation tokens for trusted peer access.",
      to: "/requests",
    },
    {
      icon: FileCheck2,
      title: "Issuer (OID4VCI)",
      description:
        "Issues W3C Verifiable Credentials to organizations and connectors following OpenID4VCI.",
      to: "/requests",
    },
    {
      icon: ShieldCheck,
      title: "Verifier (OID4VP)",
      description:
        "Requests and validates cryptographic Verifiable Presentations from holders prior to issuance.",
      to: "/requests",
    },
    ...(walletActive
      ? [
          {
            icon: Wallet,
            title: "Embedded Wallet",
            description:
              "Manage cryptographic keys, Decentralized Identifiers (DIDs), and stored Verifiable Credentials.",
            to: "/wallet/info",
          },
        ]
      : []),
  ];

  return (
    <PageLayout>
      <PageHeader
        title="Welcome to Heimdall"
        description="Self-Sovereign Identity (SSI) Authority and Embedded Wallet Manager for sovereign dataspace ecosystems."
      />

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 w-full">
        <StatTile
          label="Pending Approvals"
          value={loadingRequests ? undefined : pendingCount}
          hint={pendingCount > 0 ? "Awaiting operator decision" : "All caught up"}
          icon={Clock}
          to="/requests"
        />
        <StatTile
          label="Total VC Requests"
          value={loadingRequests ? undefined : requests.length}
          icon={ClipboardCheck}
          to="/requests"
        />
        <StatTile
          label="Participants"
          value={loadingParticipants ? undefined : participants.length}
          icon={Users}
          to="/participants"
        />
        {walletActive && (
          <StatTile
            label="Wallet Credentials"
            value={loadingVcs ? undefined : vcs.length}
            icon={Wallet}
            to="/wallet/credentials"
          />
        )}
      </div>

      <PageSection title="Core Capabilities">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {features.map((feature) => (
            <FeatureCard
              key={feature.title}
              icon={feature.icon}
              title={feature.title}
              description={feature.description}
              to={feature.to}
            />
          ))}
        </div>
      </PageSection>
    </PageLayout>
  );
}
