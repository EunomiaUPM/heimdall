// Generate the API reference pages from the OpenAPI spec:
//   npm run generate:api
import { rm, writeFile } from 'node:fs/promises';
import { generateFiles } from 'fumadocs-openapi';
import { createOpenAPI } from 'fumadocs-openapi/server';

const out = './content/docs/api/reference';

await rm(out, { recursive: true, force: true });
await generateFiles({
  input: createOpenAPI({ input: { authority: '../static/specs/openapi/openapi.json' } }),
  output: out,
  per: 'tag',
  includeDescription: true,
});
await writeFile(`${out}/meta.json`, JSON.stringify({ title: 'Reference', pages: ['...'] }, null, 2) + '\n');
