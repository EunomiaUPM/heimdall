import Link from 'next/link';
import type { Metadata } from 'next';
import { Architecture, Issuance } from '@/components/diagrams';
import { ArrowIcon, GitHubIcon } from '@/components/landing/icons';
import { ButtonLink, Glow, Heading, Lead, Marquee, Section, Terminal, type TerminalLine } from '@/components/landing/primitives';
import { asset, site, standards } from '@/lib/site';

export const metadata: Metadata = {
  title: { absolute: site.title },
  description: site.description,
};

const start: TerminalLine[] = [
  { kind: 'cmd', text: 'git clone https://github.com/EunomiaUPM/heimdall.git' },
  { kind: 'cmd', text: 'cd heimdall && git checkout v1.5.1' },
  { kind: 'cmd', text: 'docker compose up -d' },
  { kind: 'cmd', text: 'curl -X POST 127.0.0.1:1500/api/v1/wallet/link' },
  { kind: 'cmd', text: 'curl 127.0.0.1:1500/.well-known/did.json' },
  { kind: 'info', text: 'console at 127.0.0.1:1500/admin' },
];

const jobs = [
  {
    title: 'Decide',
    body: 'Every credential request starts as a signed GNAP grant. The authority approves it automatically, straight away or after a presentation.',
  },
  {
    title: 'Verify',
    body: 'When a credential depends on others, the holder presents them over OID4VP and the authority checks them before going on.',
  },
  {
    title: 'Issue',
    body: 'Once approved, the holder’s wallet redeems an OID4VCI offer and receives the credential, signed with the authority’s key.',
  },
];

const roles = [
  { name: 'DataSpaceAuthority', body: 'Membership of the data space.' },
  { name: 'LegalAuthority', body: 'Legal identifiers read from the holder’s certificate.' },
  { name: 'ClearingHouse', body: 'Imitates a Gaia-X clearing house: checks the credentials presented and issues a label.' },
  { name: 'EcoAuthority', body: 'All of the above in one instance.' },
];

export default function HomePage() {
  return (
    <main className="relative flex flex-1 flex-col overflow-hidden">
      <div className="relative">
        <div aria-hidden className="bg-grid absolute inset-0 -z-10" />
        <Glow className="-top-40 left-1/2 h-[520px] w-[820px] -translate-x-1/2" />
        <Section className="grid items-center gap-14 pb-16 pt-20 md:pt-28 lg:grid-cols-[1.05fr_1fr]">
          <div>
            <p className="mb-6 inline-flex items-center gap-2 rounded-full border border-fd-border bg-fd-card/60 px-3 py-1 font-mono text-xs text-fd-muted-foreground">
              <span className="size-1.5 rounded-full bg-brand-sky" />
              {site.version}
            </p>
            <Heading as="h1" first="Heimdall." second="Who belongs to the data space." />
            <Lead>{site.description}</Lead>
            <div className="mt-10 flex flex-wrap gap-3">
              <ButtonLink href="/docs/quickstart">
                Get started <ArrowIcon className="size-4" />
              </ButtonLink>
              <ButtonLink href={site.repo} variant="secondary" external>
                <GitHubIcon className="size-4" /> View on GitHub
              </ButtonLink>
            </div>
          </div>
          <Terminal title="~/heimdall" lines={start} />
        </Section>
      </div>

      <Marquee items={standards} />

      <Section number="01" label="What it does">
        <Heading first="Three jobs." second="One authority." />
        <div className="mt-14 grid gap-5 md:grid-cols-3">
          {jobs.map((j, i) => (
            <div key={j.title} className="card-glow p-7">
              <p className="font-mono text-xs text-fd-muted-foreground">0{i + 1}</p>
              <h3 className="mt-4 text-xl font-semibold text-fd-foreground">{j.title}</h3>
              <p className="mt-3 leading-relaxed text-fd-muted-foreground">{j.body}</p>
            </div>
          ))}
        </div>
        <div className="mt-8">
          <Issuance />
        </div>
      </Section>

      <Section number="02" label="Architecture">
        <Heading first="Small modules." second="Open protocols." />
        <div className="mt-8">
          <Architecture />
        </div>
        <Link href="/docs/architecture" className="mt-2 inline-flex items-center gap-2 text-sm font-medium text-fd-primary">
          Read the architecture <ArrowIcon className="size-4" />
        </Link>
      </Section>

      <Section number="03" label="Roles">
        <Heading first="One binary." second="The role sets what it issues." />
        <div className="mt-14 grid gap-5 md:grid-cols-2">
          {roles.map((r) => (
            <div key={r.name} className="rounded-2xl border border-fd-border bg-fd-card/60 p-7">
              <h3 className="font-mono text-base font-semibold text-fd-foreground">{r.name}</h3>
              <p className="mt-3 leading-relaxed text-fd-muted-foreground">{r.body}</p>
            </div>
          ))}
        </div>
      </Section>

      <Section label="Built at UPM">
        <Lead>{site.group}.</Lead>
        <div className="mt-10 flex flex-col items-start gap-8 rounded-2xl bg-brand-night p-8 md:flex-row md:items-center md:justify-between">
          <div className="flex flex-wrap items-center gap-10">
            <img src={asset('/brand/upm_logo_light.svg')} alt="Universidad Politécnica de Madrid" className="h-12 w-auto" />
            <img src={asset('/brand/incibe_logo_white.png')} alt="INCIBE" className="h-10 w-auto" />
          </div>
          <p className="max-w-md text-sm leading-relaxed text-white/70">{site.funding}</p>
        </div>
      </Section>

      <footer className="border-t border-fd-border">
        <div className="mx-auto flex w-full max-w-6xl flex-wrap items-center justify-between gap-4 px-6 py-10 text-sm text-fd-muted-foreground">
          <p>
            {site.name} {site.version} · {site.license}
          </p>
          <div className="flex flex-wrap gap-6">
            <Link href="/docs" className="hover:text-fd-foreground">
              Docs
            </Link>
            <a href={site.docs} className="hover:text-fd-foreground">
              EUNOMIA documentation
            </a>
            <a href={site.website} className="hover:text-fd-foreground">
              Project website
            </a>
            <a href={`mailto:${site.contact}`} className="hover:text-fd-foreground">
              Contact
            </a>
          </div>
        </div>
      </footer>
    </main>
  );
}
