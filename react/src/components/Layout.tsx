import React, { useMemo } from "react";
import { Link, Outlet, useLocation } from "react-router-dom";
import {
  Home as HomeIcon,
  Info,
  ShieldCheck,
  Users,
  Wallet,
} from "lucide-react";
import {
  AppShellHeader,
  AppShellSidebar,
  GlobalLoadingIndicator,
  type LinkProps,
  NavigationProvider,
  type NavGroup,
  ParticipantDirectoryProvider,
  type SessionUser,
  SidebarInset,
  SidebarProvider,
  Toaster,
} from "@eunomia/ui";
import { NotificationBell } from "@/components/NotificationBell";
import { useHeimdallDirectory } from "@/hooks/useHeimdallDirectory";

const ReactRouterLink = React.forwardRef<HTMLAnchorElement, LinkProps>(
  ({ to, ...rest }, ref) => <Link ref={ref} to={to} {...rest} />,
);
ReactRouterLink.displayName = "ReactRouterLink";

export interface LayoutProps {
  user: SessionUser | null;
  onSignOut?: () => void;
}

export function Layout({ user, onSignOut }: LayoutProps) {
  const location = useLocation();
  const { resolveParticipant } = useHeimdallDirectory();
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE !== "false";

  const navGroups: NavGroup[] = useMemo(() => {
    const myAreaItems = [
      ...(walletActive
        ? [
            {
              title: "My Wallet",
              url: "/wallet/info",
              section: "/wallet",
              icon: Wallet,
            },
          ]
        : []),
      {
        title: "About",
        url: "/about",
        icon: Info,
      },
    ];

    return [
      {
        title: "General",
        items: [
          {
            title: "Overview",
            url: "/home",
            icon: HomeIcon,
          },
          {
            title: "Participants",
            url: "/participants",
            icon: Users,
          },
          {
            title: "VC Requests",
            url: "/requests",
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
    <NavigationProvider link={ReactRouterLink} activePath={location.pathname}>
      <ParticipantDirectoryProvider value={resolveParticipant}>
        <SidebarProvider>
          <AppShellSidebar
            productName="Heimdall"
            homeTo="/home"
            navGroups={navGroups}
          />
          <SidebarInset>
            <AppShellHeader
              user={user}
              onSignOut={onSignOut}
              rootBreadcrumb={{ label: "Admin", href: "/home" }}
              rightActionsSlot={<NotificationBell />}
            />
            <div className="flex flex-1 flex-col gap-4 p-8 items-start justify-start w-full h-full min-w-0">
              <Outlet />
            </div>
          </SidebarInset>
        </SidebarProvider>
        <Toaster />
        <GlobalLoadingIndicator />
      </ParticipantDirectoryProvider>
    </NavigationProvider>
  );
}
