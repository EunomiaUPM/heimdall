/**
 * Facts about the project, in one place, for the landing page, the layout and
 * the metadata.
 */

/** Set when the site is served from a sub-path, e.g. GitHub Pages: /heimdall */
export const basePath = process.env.NEXT_PUBLIC_BASE_PATH ?? '';

/** A file under public/, with the base path in front. */
export function asset(path: string): string {
  return `${basePath}${path}`;
}

export const site = {
  name: 'Heimdall',
  title: 'Heimdall: the authority of an EUNOMIA data space',
  tagline: 'The authority of a data space.',
  description:
    'Heimdall onboards the participants of a data space: it issues their verifiable credentials over OID4VCI, verifies their presentations over OID4VP and decides credential requests over GNAP.',
  version: 'v1.5.1',
  url: process.env.NEXT_PUBLIC_SITE_URL ?? 'https://eunomiaupm.github.io/heimdall',
  website: 'https://eunomia.dit.upm.es',
  docs: 'https://eunomiaupm.github.io/eunomia-deployment',
  repo: 'https://github.com/EunomiaUPM/heimdall',
  license: 'GPL-3.0',
  contact: 'gi.internetng@upm.es',
  group: 'GING, Next Generation Internet Group · Department of Telematic Engineering (DIT), Universidad Politécnica de Madrid',
  funding:
    'Funded by INCIBE through the call “Solutions for sovereignty, trust and security in data spaces” (ETD202300196), co-financed by the European Union Recovery and Resilience Facility.',
};

export const standards = [
  'GNAP',
  'OID4VCI',
  'OID4VP',
  'W3C Verifiable Credentials',
  'HTTP Message Signatures',
  'did:jwk',
  'did:web',
  'Rust',
  'PostgreSQL',
  'Docker',
];
