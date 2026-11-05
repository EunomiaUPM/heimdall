# Heimdall site

The documentation of Heimdall v1.5.1, built with [Fumadocs](https://fumadocs.dev) on Next.js and exported as a static site. It is published to https://eunomiaupm.github.io/heimdall.

```bash
npm ci
npm run dev            # http://localhost:3000, with hot reload
npm run build          # static export in out/
npm run generate:api   # regenerate content/docs/api/reference/ after a spec change
npm run types:check
```

## Where things are

| Path | What |
| --- | --- |
| `content/docs/` | The documentation, one MDX file per page; `meta.json` files order the sidebar |
| `content/docs/api/reference/` | Generated from `static/specs/openapi/openapi.json`: do not edit, run `npm run generate:api` |
| `app/(home)/page.tsx` | The landing page |
| `lib/site.ts` | Project facts: name, version, links, funding |
| `components/diagrams.tsx` | The architecture diagrams |
| `app/global.css` | Theme: brand colours and tokens |

The template comes from the EUNOMIA documentation, in `eunomia-deployment`.
