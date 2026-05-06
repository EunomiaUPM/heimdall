import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Home, Users, Wallet, FileText, Info } from 'lucide-react';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@/components/ui/sidebar';
import logoImg from '@/assets/logo.svg';

export function AppSidebar() {
  const location = useLocation();
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE === 'true';
  const { state } = useSidebar();
  const isCollapsed = state === 'collapsed';

  const navGroups = [
    {
      title: 'General',
      items: [
        { title: 'Home', url: '/', icon: Home },
        { title: 'Requests', url: '/requests', icon: FileText },
        { title: 'Participants', url: '/participants', icon: Users },
        { title: 'About', url: '/about', icon: Info },
      ],
    },
    ...(walletActive
      ? [
          {
            title: 'My area',
            items: [{ title: 'My wallet', url: '/wallet', icon: Wallet }],
          },
        ]
      : []),
  ];

  const isItemActive = (url) =>
    location.pathname === url || (url !== '/' && location.pathname.startsWith(url));

  return (
    <Sidebar className="bg-base-sidebar z-50" collapsible="icon">
      <SidebarContent>
        {/* HEIMDALL title above the logo (kept from original) */}
        <div
          className={`flex flex-col items-center px-3 pt-3 transition-all duration-300 ease-in-out ${
            isCollapsed ? 'opacity-0 h-0 overflow-hidden' : 'opacity-100 h-auto'
          }`}
        >
          <div className="text-4xl font-extra tracking-widest text-center">HEIMDALL</div>
          <div className="mt-3 w-full border-t border-sidebar-border" />
        </div>

        {/* Logo (full when expanded, iso when collapsed) */}
        <div className="relative h-24 w-full flex items-center justify-center overflow-hidden">
          <img
            src={logoImg}
            className={`absolute h-16 w-auto transition-all duration-500 ease-in-out ${
              isCollapsed ? 'opacity-0 scale-50 pointer-events-none' : 'opacity-100 scale-100'
            }`}
            alt="Heimdall Logo"
          />
          <img
            src={`${import.meta.env.BASE_URL}iso_logo.svg`}
            className={`absolute h-10 w-auto transition-all duration-500 ease-in-out ${
              isCollapsed ? 'opacity-100 scale-100' : 'opacity-0 scale-50 pointer-events-none'
            }`}
            alt="Heimdall Iso"
          />
        </div>

        {navGroups.map((group) => (
          <SidebarGroup key={group.title}>
            <SidebarGroupContent>
              <SidebarGroupLabel>{group.title}</SidebarGroupLabel>
              <SidebarMenu>
                {group.items.map((item) => (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton asChild>
                      <Link
                        to={item.url}
                        className={isItemActive(item.url) ? 'bg-white/10 text-white' : ''}
                      >
                        <item.icon />
                        <span>{item.title}</span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>
    </Sidebar>
  );
}
