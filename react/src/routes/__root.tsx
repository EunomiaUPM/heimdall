import React, { useCallback, useEffect, useMemo, useState } from "react";
import { QueryClient } from "@tanstack/react-query";
import {
  createRootRouteWithContext,
  Navigate,
  Outlet,
  useNavigate,
  useRouterState,
} from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import {
  Home as HomeIcon,
  Info,
  Loader2,
  ShieldCheck,
  Users,
  Wallet,
} from "lucide-react";
import {
  AppShellHeader,
  AppShellSidebar,
  eunomiaLogoOrangeDark,
  eunomiaLogoOrangeLight,
  GeneralErrorComponent,
  GlobalLoadingIndicator,
  HEIMDALL_BUBBLE_COLORS,
  loadSession,
  type NavGroup,
  ParticipantDirectoryProvider,
  type SessionUser,
  SidebarInset,
  SidebarProvider,
  signIn,
  signOut,
  SignInScreen,
  Toaster,
  UNAUTHORIZED_EVENT,
} from "@eunomia/ui";
import { NotificationBell } from "@/components/NotificationBell";
import { NotificationProvider } from "@/contexts/NotificationContext";
import { useHeimdallDirectory } from "@/hooks/useHeimdallDirectory";

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient;
}>()({
  component: RootComponent,
  errorComponent: GeneralErrorComponent,
  notFoundComponent: () => <Navigate to="/home" replace />,
});

/** `undefined` while asking the gateway; `null` when there is no session. */
type SessionState = SessionUser | null | undefined;

const DEV_SIGNED_OUT_KEY = "heimdall.dev.signed_out";

const FALLBACK_DEV_USER: SessionUser = {
  userId: "local-operator",
  username: "operator",
  role: "/admin",
  root: true,
};

export function HeimdallSignIn({ onSignIn }: { onSignIn: () => void }) {
  return (
    <SignInScreen
      title="Eunomia Heimdall Admin"
      description="Your session has ended or has not started. Sign in to access the console."
      tagline="Trusted identity & authority anchor for sovereign data spaces."
      pillars="GNAP GateKeeper • OID4VCI Issuer • OID4VP Verifier"
      logoSrc={eunomiaLogoOrangeLight}
      bubbleColors={HEIMDALL_BUBBLE_COLORS}
      bubbleClassName="from-[#0b0b1a] via-[#15101d] to-[#080814]"
      onSignIn={onSignIn}
    />
  );
}

function AuthenticatedShell({
  user,
  onSignOut,
}: {
  user: SessionUser;
  onSignOut: () => void;
}) {
  const { resolveParticipant } = useHeimdallDirectory();
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE !== "false";

  const navGroups: NavGroup[] = useMemo(() => {
    const myAreaItems = [
      ...(walletActive
        ? [
            {
              title: "My Wallet",
              url: "/admin/wallet/info",
              section: "/admin/wallet",
              icon: Wallet,
            },
          ]
        : []),
      {
        title: "About",
        url: "/admin/about",
        icon: Info,
      },
    ];

    return [
      {
        title: "General",
        items: [
          {
            title: "Overview",
            url: "/admin/home",
            icon: HomeIcon,
          },
          {
            title: "Participants",
            url: "/admin/participants",
            icon: Users,
          },
          {
            title: "VC Requests",
            url: "/admin/requests",
            icon: ShieldCheck,
          },
        ],
      },
      {
        title: "My area",
        items: myAreaItems,
      },
    ];
  }, [walletActive]);

  return (
    <ParticipantDirectoryProvider value={resolveParticipant}>
      <NotificationProvider>
        <SidebarProvider>
          <AppShellSidebar
            productName="Heimdall"
            homeTo="/admin/home"
            navGroups={navGroups}
            logoOnDark={eunomiaLogoOrangeLight}
            logoOnLight={eunomiaLogoOrangeDark}
          />
          <SidebarInset>
            <AppShellHeader
              user={user}
              onSignOut={onSignOut}
              rightActionsSlot={<NotificationBell />}
            />
            <div className="flex flex-1 flex-col gap-4 p-8 items-start justify-start w-full h-full min-w-0">
              <Outlet />
            </div>
          </SidebarInset>
        </SidebarProvider>
        <Toaster />
        <GlobalLoadingIndicator />
        <TanStackRouterDevtools />
      </NotificationProvider>
    </ParticipantDirectoryProvider>
  );
}

function RootComponent() {
  const authDisabled = import.meta.env.VITE_AUTH_ENABLED === "false";
  const navigate = useNavigate();
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  const isLoginRoute =
    pathname.replace(/\/$/, "") === "/admin/login" ||
    pathname.replace(/\/$/, "") === "/login";

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
      void navigate({ to: "/home", replace: true });
      return;
    }
    if (authDisabled) {
      setUser(FALLBACK_DEV_USER);
      void navigate({ to: "/home", replace: true });
      return;
    }
    signIn();
  }, [authDisabled, navigate]);

  const handleSignOut = useCallback(() => {
    if (user?.userId === "system" || authDisabled) {
      sessionStorage.setItem(DEV_SIGNED_OUT_KEY, "1");
      setUser(null);
      void navigate({ to: "/login", replace: true });
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

  if (user === null || isLoginRoute) {
    return <HeimdallSignIn onSignIn={handleSignIn} />;
  }

  return <AuthenticatedShell user={user} onSignOut={handleSignOut} />;
}
