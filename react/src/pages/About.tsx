import React from "react";
import { Cpu, FileCheck2, KeyRound, ShieldCheck, Wallet } from "lucide-react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  PageHeader,
  PageLayout,
  PageSection,
} from "@eunomia/ui";

const PILLARS = [
  {
    icon: KeyRound,
    title: "GNAP GateKeeper",
    description:
      "Negotiates fine-grained access grants, interactive continuation tokens, and peer key bindings following RFC 9635 (GNAP).",
  },
  {
    icon: FileCheck2,
    title: "OID4VCI Credential Issuer",
    description:
      "Issues cryptographically signed W3C Verifiable Credentials using pre-authorized codes and proof-of-possession checks.",
  },
  {
    icon: ShieldCheck,
    title: "OID4VP Verifier",
    description:
      "Challenges holders with presentation definitions and verifies cryptographic signatures, DIDs, and X.509 trust chains.",
  },
  {
    icon: Wallet,
    title: "Embedded SSI Wallet",
    description:
      "Manages keys, Decentralized Identifiers (did:web / did:jwk), and verifiable credentials powered by the shared Ymir SSI engine.",
  },
];

export default function About() {
  return (
    <PageLayout>
      <PageHeader
        title="About Heimdall"
        description="Self-Sovereign Identity (SSI) Authority and Trust Anchor for Eunomia sovereign data spaces."
      />

      <Card className="bg-background-200/15">
        <CardHeader className="pb-2">
          <CardTitle className="text-base font-semibold flex items-center gap-2">
            <Cpu className="h-4 w-4 text-brand-sky" />
            Architecture Overview
          </CardTitle>
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground leading-relaxed space-y-3">
          <p>
            <strong className="text-foreground">Heimdall</strong> acts as the
            Dataspace Authority and Trust Service Provider in the Eunomia
            ecosystem. Built in Rust on top of the shared{" "}
            <strong className="text-foreground">Ymir</strong> SSI core, it
            governs participant onboarding, credential verification, operator
            approval workflows, and standards-compliant credential issuance.
          </p>
          <p>
            Dataspace connectors (<code className="text-brand-sky">ds-agent</code>)
            authenticate against Heimdall via GNAP, present existing credentials
            over OpenID4VP, and obtain dataspace membership and role credentials
            over OpenID4VCI.
          </p>
        </CardContent>
      </Card>

      <PageSection title="Protocol Modules">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {PILLARS.map((pillar) => {
            const Icon = pillar.icon;
            return (
              <Card key={pillar.title}>
                <CardHeader className="pb-2">
                  <CardTitle className="text-base font-semibold flex items-center gap-2">
                    <Icon className="h-4 w-4 text-brand-sky" />
                    {pillar.title}
                  </CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground leading-relaxed">
                  {pillar.description}
                </CardContent>
              </Card>
            );
          })}
        </div>
      </PageSection>
    </PageLayout>
  );
}
