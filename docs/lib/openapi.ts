import { createOpenAPI } from 'fumadocs-openapi/server';

/** The spec the API reference is generated from, by document id. */
export const openapiInputs = {
  authority: '../static/specs/openapi/openapi.json',
};

export const openapi = createOpenAPI({
  input: openapiInputs,
});
