import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig(() => {
  const gatewayPort =
    process.env.VITE_GATEWAY_PORT || process.env.VITE_BACKEND_PORT || "1500";
  const backendTarget =
    process.env.VITE_GATEWAY_TARGET ||
    process.env.VITE_BACKEND_TARGET ||
    `http://127.0.0.1:${gatewayPort}`;
  const devPort = Number(process.env.VITE_PORT) || 5176;

  return {
    base: "/admin/",
    plugins: [react()],
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
      dedupe: ["react", "react-dom", "react/jsx-runtime", "@tanstack/react-query"],
    },
    optimizeDeps: {
      exclude: ["lucide-react", "@eunomia/ui"],
    },
    server: {
      port: devPort,
      proxy: {
        "/api": {
          target: backendTarget,
          changeOrigin: true,
        },
        "/oauth2": {
          target: backendTarget,
          changeOrigin: true,
        },
        "/.well-known": {
          target: backendTarget,
          changeOrigin: true,
        },
      },
    },
  };
});
