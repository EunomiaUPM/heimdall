import React, { useEffect, useState } from "react";
import { Navigate, Outlet, Route, Routes } from "react-router-dom";
import {
  loadSession,
  type SessionUser,
  signOut,
  SignInScreen,
  UNAUTHORIZED_EVENT,
  WalletCredentialsView,
  WalletDidView,
  WalletInfoView,
  WalletKeysView,
  WalletLayout,
  WalletOidc4vciView,
  WalletOidc4vpView,
} from "@eunomia/ui";
import { Layout } from "@/components/Layout";
import { NotificationProvider } from "@/contexts/NotificationContext";
import Home from "@/pages/Home";
import About from "@/pages/About";
import Participants from "@/pages/Participants";
import ParticipantDetails from "@/pages/ParticipantDetails";
import Requests from "@/pages/Requests";
import RequestDetails from "@/pages/RequestDetails";

const FALLBACK_DEV_USER: SessionUser = {
  userId: "local-operator",
  username: "operator",
  role: "AuthorityAdmin",
  root: true,
};

function WalletRouteShell() {
  return (
    <WalletLayout basePath="/wallet">
      <Outlet />
    </WalletLayout>
  );
}

export default function App() {
  const authDisabled = import.meta.env.VITE_AUTH_ENABLED === "false";
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE !== "false";

  const [user, setUser] = useState<SessionUser | null>(null);
  const [checkingSession, setCheckingSession] = useState(true);
  const [unauthorized, setUnauthorized] = useState(false);

  useEffect(() => {
    let active = true;

    loadSession()
      .then((resolved) => {
        if (!active) return;
        if (resolved) {
          setUser(resolved);
          setUnauthorized(false);
        } else if (authDisabled) {
          setUser(FALLBACK_DEV_USER);
          setUnauthorized(false);
        } else {
          setUser(null);
          setUnauthorized(true);
        }
      })
      .finally(() => {
        if (active) setCheckingSession(false);
      });

    return () => {
      active = false;
    };
  }, [authDisabled]);

  useEffect(() => {
    const onUnauthorized = () => {
      if (!authDisabled) {
        setUser(null);
        setUnauthorized(true);
      }
    };
    window.addEventListener(UNAUTHORIZED_EVENT, onUnauthorized);
    return () => window.removeEventListener(UNAUTHORIZED_EVENT, onUnauthorized);
  }, [authDisabled]);

  if (checkingSession) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-background text-muted-foreground text-sm">
        Loading Heimdall console…
      </div>
    );
  }

  if (unauthorized || !user) {
    return (
      <SignInScreen
        title="Eunomia Heimdall Admin"
        description="Sign in with your authority operator credentials to manage participants, credential requests, and wallet keys."
        pillars="GNAP GateKeeper • OID4VCI Issuer • OID4VP Verifier"
      />
    );
  }

  return (
    <NotificationProvider>
      <Routes>
        <Route
          path="/"
          element={
            <Layout
              user={user}
              onSignOut={authDisabled ? undefined : signOut}
            />
          }
        >
          <Route index element={<Navigate to="/home" replace />} />
          <Route path="home" element={<Home />} />
          <Route path="about" element={<About />} />
          <Route path="participants" element={<Participants />} />
          <Route path="participants/:id" element={<ParticipantDetails />} />
          <Route path="requests" element={<Requests />} />
          <Route path="requests/:id" element={<RequestDetails />} />

          {walletActive && (
            <Route path="wallet" element={<WalletRouteShell />}>
              <Route index element={<Navigate to="info" replace />} />
              <Route path="info" element={<WalletInfoView />} />
              <Route path="did" element={<WalletDidView />} />
              <Route path="keys" element={<WalletKeysView />} />
              <Route
                path="credentials"
                element={
                  <WalletCredentialsView
                    showGaiaCompliance={false}
                    requestVcHref={() => "/requests"}
                  />
                }
              />
              <Route path="oidc4vp" element={<WalletOidc4vpView />} />
              <Route path="oidc4vci" element={<WalletOidc4vciView />} />
            </Route>
          )}

          <Route path="*" element={<Navigate to="/home" replace />} />
        </Route>
      </Routes>
    </NotificationProvider>
  );
}
