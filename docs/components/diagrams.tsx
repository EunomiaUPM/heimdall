/**
 * Architecture diagrams as inline SVG. Colours come from the theme tokens, so
 * they read the same in light and dark mode.
 */
import type { ReactNode } from 'react';

type BoxProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  subtitle?: string;
  chip?: string;
  accent?: boolean;
};

function Box({ x, y, w, h, title, subtitle, chip, accent }: BoxProps) {
  return (
    <g>
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={12}
        className={accent ? 'fill-fd-accent stroke-fd-primary' : 'fill-fd-card stroke-fd-border'}
        strokeWidth={accent ? 1.5 : 1}
      />
      <text x={x + 16} y={y + 28} className="fill-fd-foreground" fontSize={15} fontWeight={600}>
        {title}
      </text>
      {subtitle && (
        <text x={x + 16} y={y + 48} className="fill-fd-muted-foreground" fontSize={12}>
          {subtitle}
        </text>
      )}
      {chip && (
        <g>
          <rect
            x={x + w - 92}
            y={y + h - 26}
            width={80}
            height={18}
            rx={9}
            className="fill-fd-secondary stroke-fd-border"
            strokeWidth={1}
          />
          <text x={x + w - 52} y={y + h - 13} textAnchor="middle" className="fill-fd-muted-foreground" fontSize={10}>
            {chip}
          </text>
        </g>
      )}
    </g>
  );
}

function Arrow({ d, label, lx, ly, dashed, anchor = 'middle' }: {
  d: string;
  label?: string;
  lx?: number;
  ly?: number;
  dashed?: boolean;
  anchor?: 'start' | 'middle' | 'end';
}) {
  return (
    <g>
      <path
        d={d}
        fill="none"
        className="stroke-fd-muted-foreground"
        strokeWidth={1.4}
        strokeDasharray={dashed ? '5 5' : undefined}
        markerEnd="url(#arrow)"
      />
      {label && lx !== undefined && ly !== undefined && (
        <text x={lx} y={ly} textAnchor={anchor} className="fill-fd-muted-foreground" fontSize={11}>
          {label}
        </text>
      )}
    </g>
  );
}

function Frame({ viewBox, title, children }: { viewBox: string; title: string; children: ReactNode }) {
  return (
    <figure className="not-prose my-6 overflow-x-auto rounded-xl border bg-fd-background p-4">
      <svg viewBox={viewBox} role="img" aria-label={title} className="h-auto w-full min-w-[560px] font-sans">
        <title>{title}</title>
        <defs>
          <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
            <path d="M 0 0 L 10 5 L 0 10 z" className="fill-fd-muted-foreground" />
          </marker>
        </defs>
        {children}
      </svg>
    </figure>
  );
}

/** The modules of the authority and what they talk to. */
export function Architecture() {
  return (
    <Frame viewBox="0 0 760 420" title="Heimdall: its modules, the participants that call it and the services it depends on">
      <Box x={30} y={16} w={300} h={70} title="Participant" subtitle="agent and its wallet" />
      <Box x={430} y={16} w={300} h={70} title="Operator" subtitle="admin console at /admin" />

      <rect x={30} y={126} width={700} height={160} rx={14} className="fill-fd-accent stroke-fd-primary" strokeWidth={1.5} />
      <text x={50} y={154} className="fill-fd-foreground" fontSize={15} fontWeight={600}>
        Heimdall
      </text>
      <Box x={50} y={170} w={150} h={50} title="Gatekeeper" />
      <Box x={214} y={170} w={150} h={50} title="Verifier" />
      <Box x={378} y={170} w={150} h={50} title="Issuer" />
      <Box x={542} y={170} w={170} h={50} title="Approver" />
      <Box x={50} y={228} w={314} h={46} title="Credential builder, chosen by role" />
      <Box x={378} y={228} w={334} h={46} title="Participants and federated catalog" />

      <Box x={30} y={334} w={210} h={70} title="PostgreSQL" subtitle="grants, issuances, participants" />
      <Box x={275} y={334} w={210} h={70} title="Fafnir" subtitle="keys, DID and signatures" />
      <Box x={520} y={334} w={210} h={70} title="Vault" subtitle="secrets and certificates" />

      <Arrow d="M 180 90 L 180 122" label="GNAP · OID4VP · OID4VCI" lx={190} ly={110} anchor="start" />
      <Arrow d="M 580 90 L 580 122" label="REST" lx={590} ly={110} anchor="start" />
      <Arrow d="M 135 290 L 135 330" />
      <Arrow d="M 380 290 L 380 330" />
      <Arrow d="M 625 290 L 625 330" />
    </Frame>
  );
}

/** The four steps of a credential request. */
export function Issuance() {
  const steps = [
    { n: '1', t: 'Ask', s: 'POST /gate/access', d: 'signed GNAP grant' },
    { n: '2', t: 'Prove', s: 'cert or OID4VP', d: 'who the holder is' },
    { n: '3', t: 'Decide', s: 'automatic', d: 'once the proof holds' },
    { n: '4', t: 'Redeem', s: 'POST /issuer/credential', d: 'OID4VCI, by the wallet' },
  ];
  return (
    <Frame viewBox="0 0 760 150" title="A credential request: ask, prove, decide, redeem">
      {steps.map((st, i) => {
        const x = 16 + i * 186;
        return (
          <g key={st.n}>
            <rect x={x} y={20} width={168} height={108} rx={12} className="fill-fd-card stroke-fd-border" strokeWidth={1} />
            <circle cx={x + 26} cy={48} r={13} className="fill-fd-accent stroke-fd-primary" strokeWidth={1.2} />
            <text x={x + 26} y={52} textAnchor="middle" className="fill-fd-foreground" fontSize={12} fontWeight={700}>
              {st.n}
            </text>
            <text x={x + 48} y={53} className="fill-fd-foreground" fontSize={15} fontWeight={600}>
              {st.t}
            </text>
            <text x={x + 16} y={88} className="fill-fd-foreground font-mono" fontSize={11}>
              {st.s}
            </text>
            <text x={x + 16} y={110} className="fill-fd-muted-foreground" fontSize={11}>
              {st.d}
            </text>
            {i < steps.length - 1 && <Arrow d={`M ${x + 170} 74 L ${x + 184} 74`} />}
          </g>
        );
      })}
    </Frame>
  );
}
