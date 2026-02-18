import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { cn } from '@/lib/utils';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Button } from '@/components/ui/button';
import { Bell, User } from 'lucide-react';
import { SidebarTrigger } from '@/components/ui/sidebar';
import { Separator } from '@/components/ui/separator';

export const Header = () => {
  const location = useLocation();

  const generateBreadcrumbs = () => {
    const segments = location.pathname.split('/').filter(Boolean);

    // On root "/" or "/home" show just "Home"
    if (segments.length === 0 || (segments.length === 1 && segments[0] === 'home')) {
      return [{ key: '/', href: '/home', label: 'Home', isLast: true }];
    }

    // For deeper paths, build crumbs without prepending Home
    return segments.map((segment, index) => {
      const url = `/${segments.slice(0, index + 1).join('/')}`;
      const isLast = index === segments.length - 1;
      return {
        key: url,
        href: url,
        label: segment.charAt(0).toUpperCase() + segment.slice(1),
        isLast,
      };
    });
  };

  const breadcrumbs = generateBreadcrumbs();

  return (
    <div className="bg-background w-full border-b border-white/5 py-1 z-50 h-14 px-4 flex justify-between items-center gap-4">
      {/* Left section: Sidebar trigger and breadcrumbs */}
      <div className="flex items-center gap-2 overflow-hidden min-w-0">
        <SidebarTrigger className="h-8 w-8 shrink-0" />
        <Separator orientation="vertical" className="h-6 shrink-0" />
        <Breadcrumb className="overflow-hidden">
          <BreadcrumbList className="flex-nowrap">
            {breadcrumbs.map((item, index) => (
              <React.Fragment key={item.key}>
                {index > 0 && <BreadcrumbSeparator />}
                <BreadcrumbItem className="whitespace-nowrap">
                  {item.isLast ? (
                    <BreadcrumbPage>{item.label}</BreadcrumbPage>
                  ) : (
                    <BreadcrumbLink asChild>
                      <Link to={item.href}>{item.label}</Link>
                    </BreadcrumbLink>
                  )}
                </BreadcrumbItem>
              </React.Fragment>
            ))}
          </BreadcrumbList>
        </Breadcrumb>
      </div>

      {/* Right section: User actions */}
      <div className="flex flex-row gap-4 shrink-0 items-center">
        <Button variant="ghost" size="icon" className="h-9 w-9">
          <Bell className="h-5 w-5 text-muted-foreground" />
        </Button>
        <Button variant="ghost" size="icon" className="h-9 w-9">
          <User className="h-5 w-5 text-muted-foreground" />
        </Button>
      </div>
    </div>
  );
};
