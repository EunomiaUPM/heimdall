import { defineConfig } from "orval";

export default defineConfig({
  heimdall: {
    input: {
      target: "../static/specs/openapi/openapi.json",
    },
    output: {
      mode: "tags-split",
      target: "src/data/orval/heimdall.ts",
      schemas: "src/data/orval/model",
      client: "react-query",
      mock: false,
      override: {
        mutator: {
          path: "src/data/orval-mutator.ts",
          name: "customInstance",
        },
      },
    },
  },
});
