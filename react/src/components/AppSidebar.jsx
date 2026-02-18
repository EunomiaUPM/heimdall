import React from 'react';
// 1. Asegúrate de que useLocation esté importado de react-router-dom
import { Link, useLocation } from 'react-router-dom';
import { Home, Users, Wallet, FileText, Info } from 'lucide-react';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@/components/ui/sidebar';
import logoImg from '@/assets/logo.svg';

export function AppSidebar() {
  const location = useLocation(); // Esto ya no debería dar error
  const walletActive = import.meta.env.VITE_WALLET_ACTIVE === 'true';
  const { state } = useSidebar();
  const isCollapsed = state === 'collapsed';

  const items = [
    { title: 'Home', url: '/', icon: Home },
    { title: 'Requests', url: '/requests', icon: FileText },
    { title: 'Minions', url: '/minions', icon: Users },
    ...(walletActive ? [{ title: 'Wallet', url: '/wallet', icon: Wallet }] : []),
    { title: 'About', url: '/about', icon: Info },
  ];

  return (
    <Sidebar className="bg-base-sidebar" collapsible="icon">
      <SidebarContent>
        <SidebarGroup>
          {/* Título HEIMDALL: Se oculta suavemente */}
          <div
            className={`flex flex-col items-center px-3 pt-3 transition-all duration-300 ease-in-out ${
              isCollapsed ? 'opacity-0 h-0 overflow-hidden' : 'opacity-100 h-auto'
            }`}
          >
            <div className="text-4xl font-extra tracking-widest text-center">HEIMDALL</div>
            <div className="mt-3 w-full border-t border-sidebar-border" />
          </div>

          {/* Contenedor de Logos: Mantiene el espacio para evitar saltos */}
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
