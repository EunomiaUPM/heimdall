import { ShieldCheck, FileCheck2, KeyRound, ClipboardCheck, Wallet } from 'lucide-react';

const features = [
  {
    icon: KeyRound,
    title: 'GateKeeper',
    description:
      'Controls who can access what. Manages fine-grained authorization so only the right parties get through.',
  },
  {
    icon: FileCheck2,
    title: 'Issuer',
    description:
      'Issues Verifiable Credentials to individuals and organizations, following open identity standards.',
  },
  {
    icon: ShieldCheck,
    title: 'Verifier',
    description:
      'Validates credentials and presentations provided by holders, ensuring their authenticity.',
  },
  {
    icon: ClipboardCheck,
    title: 'Approver',
    description:
      'Manages approval workflows for credential requests, giving administrators full control over issuance.',
  },
  {
    icon: Wallet,
    title: 'Wallet',
    description:
      'An embedded wallet for managing cryptographic keys and decentralized identifiers (DIDs).',
  },
];

const Home = () => {
  return (
    <div className="p-6 max-w-3xl">
      {/* Hero */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight mb-2">🛡️ Welcome to Heimdall</h1>
        <p className="text-muted-foreground text-base leading-relaxed">
          Heimdall is a <strong>Self-Sovereign Identity (SSI) Authority</strong> and{' '}
          <strong>Wallet Manager</strong>. It acts as a central pillar in a digital identity
          ecosystem, enabling the issuance, verification, and management of{' '}
          <strong>Verifiable Credentials (VCs)</strong> and{' '}
          <strong>Verifiable Presentations (VPs)</strong> — putting users in control of their own
          identity data.
        </p>
      </div>

      {/* Feature cards */}
      <h2 className="text-lg font-semibold mb-4 text-foreground/80">What Heimdall does</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {features.map(({ icon: Icon, title, description }) => (
          <div
            key={title}
            className="rounded-xl border border-white/10 bg-white/5 p-4 flex gap-3 items-start hover:bg-white/10 transition-colors"
          >
            <div className="mt-0.5 shrink-0 text-primary">
              <Icon className="h-5 w-5" />
            </div>
            <div>
              <p className="font-semibold text-sm mb-1">{title}</p>
              <p className="text-xs text-muted-foreground leading-relaxed">{description}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Home;
