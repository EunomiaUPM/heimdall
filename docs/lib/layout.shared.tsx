import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { asset, site } from './site';

export function Logo() {
  return (
    <>
      <img src={asset('/brand/eunomia_logo_lg_dark.svg')} alt="EUNOMIA" className="h-6 w-auto dark:hidden" />
      <img src={asset('/brand/eunomia_logo_lg_light.svg')} alt={site.name} className="hidden h-6 w-auto dark:block" />
      <span className="font-semibold">Heimdall</span>
    </>
  );
}

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: <Logo />,
      url: '/',
    },
    githubUrl: site.repo,
    links: [
      { text: 'Docs', url: '/docs', active: 'nested-url' },
      { text: 'Architecture', url: '/docs/architecture' },
      { text: 'API', url: '/docs/api' },
    ],
  };
}
