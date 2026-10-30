import React, { useCallback, useEffect, useState } from "react";
import { Navigate, Outlet, Route, Routes, useNavigate } from "react-router-dom";
import { Loader2 } from "lucide-react";
import {
  eunomiaLogoOrangeLight,
  HEIMDALL_BUBBLE_COLORS,
  loadSession,
  type SessionUser,
  signIn,
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

/** `undefined` while asking the gateway; `null` when there is no session. */
type SessionState = SessionUser | null | undefined;

const DEV_SIGNED_OUT_KEY = "heimdall.dev.signed_out";

const FALLBACK_DEV_USER: SessionUser = {
  userId: "local-operator",
  username: "operator",
  role: "/admin",
  root: true,
};

function HeimdallSignIn({ onSignIn }: { onSignIn: () => void }) {
  return (
    <SignInScreen
      title="Eunomia Heimdall Admin"
      description="Your session has ended or has not started. Sign in to access the console."
      tagline="Trusted identity & authority anchor for sovereign data spaces."
      pillars="GNAP GateKeeper • OID4VCI Issuer • OID4VP Verifier"
      logoSrc={eunomiaLogoOrangeLight}
      bubbleColors={HEIMDALL_BUBBLE_COLORS}
      bubbleClassName="from-[#140b07] via-[#1f1009] to-[#0a0812]"
      onSignIn={onSignIn}
    />
  );
}

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
  const navigate = useNavigate();

  const [user, setUser] = useState<SessionState>(undefined);

  useEffect(() => {
    let active = true;

    loadSession().then((resolved) => {
      if (!active) return;
      if (resolved) {
        const devSignedOut =
          resolved.userId === "system" &&
          sessionStorage.getItem(DEV_SIGNED_OUT_KEY) === "1";
        setUser(devSignedOut ? null : resolved);
      } else if (authDisabled) {
        const devSignedOut = sessionStorage.getItem(DEV_SIGNED_OUT_KEY) === "1";
        setUser(devSignedOut ? null : FALLBACK_DEV_USER);
      } else {
        setUser(null);
      }
    });

    const handleUnauthorized = () => {
      if (!authDisabled) {
        setUser(null);
      }
    };
    window.addEventListener(UNAUTHORIZED_EVENT, handleUnauthorized);
    return () => {
      active = false;
      window.removeEventListener(UNAUTHORIZED_EVENT, handleUnauthorized);
    };
  }, [authDisabled]);

  const handleSignIn = useCallback(async () => {
    sessionStorage.removeItem(DEV_SIGNED_OUT_KEY);
    const resolved = await loadSession();
    if (resolved) {
      setUser(resolved);
      navigate("/home", { replace: true });
      return;
    }
    if (authDisabled) {
      setUser(FALLBACK_DEV_USER);
      navigate("/home", { replace: true });
      return;
    }
    signIn();
  }, [authDisabled, navigate]);

  const handleSignOut = useCallback(() => {
    if (user?.userId === "system" || authDisabled) {
      sessionStorage.setItem(DEV_SIGNED_OUT_KEY, "1");
      setUser(null);
      navigate("/login", { replace: true });
      return;
    }
    signOut("/admin/");
  }, [authDisabled, navigate, user]);

  if (user === undefined) {
    return (
      <div className="flex h-screen w-full items-center justify-center bg-background">
        <Loader2 className="h-8 w-8 animate-spin text-brand-sky" />
      </div>
    );
  }

  if (user === null) {
    return <HeimdallSignIn onSignIn={handleSignIn} />;
  }

  return (
    <NotificationProvider>
      <Routes>
        <Route
          path="/login"
          element={<HeimdallSignIn onSignIn={handleSignIn} />}
        />
        <Route
          path="/"
          element={<Layout user={user} onSignOut={handleSignOut} />}
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
