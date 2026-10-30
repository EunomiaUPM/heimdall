import React from "react";
import ReactDOM from "react-dom/client";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import {
  GeneralErrorComponent,
  setApiGatewayBase,
  ThemeContextProvider,
} from "@eunomia/ui";
import { routeTree } from "./routeTree.gen";
import "./index.css";

setApiGatewayBase(import.meta.env.VITE_API_PATH || "/api/v1");

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 15_000,
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

const router = createRouter({
  routeTree,
  context: { queryClient },
  basepath: "/admin",
  defaultPreload: "intent",
  defaultErrorComponent: GeneralErrorComponent,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <ThemeContextProvider defaultTheme="dark">
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </ThemeContextProvider>,
);
