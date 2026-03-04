import React from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
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
import { Bell, User, Trash2 } from 'lucide-react';
import { SidebarTrigger } from '@/components/ui/sidebar';
import { Separator } from '@/components/ui/separator';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useNotifications } from '@/contexts/NotificationContext';

export const Header = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { notifications, unreadCount, markAllAsRead, clearNotifications } = useNotifications();

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
        <DropdownMenu
          onOpenChange={(isOpen) => {
            if (isOpen) markAllAsRead();
          }}
        >
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon" className="h-9 w-9 relative">
              <Bell className="h-5 w-5 text-muted-foreground" />
              {unreadCount > 0 && (
                <span className="absolute top-1 right-1 flex h-3 w-3 items-center justify-center rounded-full bg-danger text-[9px] text-white">
                  {unreadCount > 9 ? '9+' : unreadCount}
                </span>
              )}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="w-80 max-h-[400px] overflow-y-auto">
            <DropdownMenuLabel className="flex items-center justify-between">
              Notifications
              {notifications.length > 0 && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={clearNotifications}
                  className="h-6 px-2 text-xs text-muted-foreground hover:text-danger"
                >
                  <Trash2 className="h-3 w-3 mr-1" /> Clear
                </Button>
              )}
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            {notifications.length === 0 ? (
              <div className="py-8 text-center text-sm text-muted-foreground">
                No new notifications
              </div>
            ) : (
              notifications.map((notif, idx) => (
                <DropdownMenuItem
                  key={idx}
                  className="flex flex-col items-start p-3 gap-1 cursor-pointer"
                  onClick={() => {
                    if (notif.link) navigate(notif.link);
                  }}
                >
                  <div className="flex w-full items-center justify-between">
                    <span className="font-semibold text-sm">{notif.title}</span>
                    <span className="text-xs text-muted-foreground">
                      {new Date(notif.created_at).toLocaleTimeString()}
                    </span>
                  </div>
                  <span className="text-sm text-muted-foreground line-clamp-2">
                    {notif.message}
                  </span>
                </DropdownMenuItem>
              ))
            )}
          </DropdownMenuContent>
        </DropdownMenu>
        <Button variant="ghost" size="icon" className="h-9 w-9">
          <User className="h-5 w-5 text-muted-foreground" />
        </Button>
      </div>
    </div>
  );
};
