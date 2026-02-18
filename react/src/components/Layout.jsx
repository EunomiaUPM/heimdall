import React from 'react';
import { Outlet } from 'react-router-dom';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { AppSidebar } from './AppSidebar';
import { Header } from './Header';

const Layout = () => {
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <Header />
        {/* Using CSS variables or utility classes to match the previous dark theme background if needed.
             The index.css defines body bg-background, so SidebarInset (which has bg-background) should match.
             We just need to ensure the padding and overflow are correct.
         */}
        <div className="flex flex-1 flex-col gap-4 p-8 pt-6 overflow-hidden items-start justify-start w-full h-full">
          <Outlet />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
};

export default Layout;
