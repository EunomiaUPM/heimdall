import { Home, Users, Wallet, FileText, Info } from 'lucide-react';
import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import logoImg from '@/assets/logo.svg';

export function AppSidebar() {
  const location = useLocation();
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE === 'true';

  const items = [
    {
      title: 'Home',
      url: '/',
      icon: Home,
    },
    {
      title: 'Requests',
      url: '/requests',
      icon: FileText,
    },
    {
      title: 'Minions',
      url: '/minions',
      icon: Users,
    },
    ...(walletActive
      ? [
          {
            title: 'Wallet',
            url: '/wallet',
            icon: Wallet,
          },
        ]
      : []),
    {
      title: 'About',
      url: '/about',
      icon: Info,
    },
  ];

  return (
    <Sidebar className="bg-base-sidebar" collapsible="icon">
      <SidebarContent>
        <SidebarGroup>
          {/* Logo */}
          <div className="flex h-16 items-center px-4">
            <img
              src={logoImg}
              className="h-8 object-contain transition-all group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:w-8"
              alt="Eunomia Logo"
            />
          </div>

          {/* Navigation Menu */}
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton
                    asChild
                    isActive={
                      location.pathname === item.url ||
                      (item.url !== '/' && location.pathname.startsWith(item.url))
                    }
                  >
                    <Link to={item.url}>
                      <item.icon />
                      <span>{item.title}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
