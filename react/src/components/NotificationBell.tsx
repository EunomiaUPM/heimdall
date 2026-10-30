import React, { useState } from "react";
import { Link, useNavigate } from "@tanstack/react-router";
import { Bell, CheckCheck } from "lucide-react";
import {
  cn,
  encodePathId,
  Popover,
  PopoverContent,
  PopoverTrigger,
  RelativeTime,
} from "@eunomia/ui";
import {
  type HeimdallNotification,
  useNotifications,
} from "@/contexts/NotificationContext";

export function NotificationBell() {
  const [open, setOpen] = useState(false);
  const navigate = useNavigate();
  const { notifications, unreadCount, connected, markAsRead, markAllAsRead } =
    useNotifications();

  const handleOpenNotification = (item: HeimdallNotification) => {
    markAsRead(item.id);
    setOpen(false);
    if (item.requestId) {
      void navigate({
        to: "/requests/$requestId",
        params: { requestId: item.requestId },
      });
    } else if (item.participantId) {
      void navigate({
        to: "/participants/$participantId",
        params: { participantId: encodePathId(item.participantId) },
      });
    } else {
      void navigate({ to: "/requests" });
    }
  };

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <button
          type="button"
          className="relative rounded-md p-1.5 text-muted-foreground transition-colors hover:bg-ink/5 hover:text-foreground"
          aria-label={
            unreadCount ? `${unreadCount} unread notifications` : "Notifications"
          }
        >
          <Bell className="h-4 w-4" />
          {unreadCount > 0 && (
            <span className="absolute -right-1 -top-1 flex h-4 min-w-4 items-center justify-center rounded-full bg-danger-600 px-1 text-[0.625rem] font-semibold leading-none text-white tabular-nums">
              {unreadCount > 99 ? "99+" : unreadCount}
            </span>
          )}
        </button>
      </PopoverTrigger>
      <PopoverContent
        align="end"
        sideOffset={8}
        className="z-[100] w-96 overflow-hidden rounded-xl border border-ink/15 bg-background-800/95 p-0 shadow-2xl backdrop-blur-xl"
      >
        <div className="flex items-center justify-between border-b border-ink/10 px-4 py-3">
          <div className="flex items-center gap-2">
            <span className="text-sm font-semibold">Notifications</span>
            <span
              className={cn(
                "h-1.5 w-1.5 rounded-full",
                connected ? "bg-success-500" : "bg-ink/30",
              )}
              title={connected ? "Live SSE stream connected" : "Live stream reconnecting"}
            />
          </div>
          <button
            type="button"
            onClick={markAllAsRead}
            disabled={unreadCount === 0}
            className="inline-flex items-center gap-1 text-xs text-brand-sky hover:underline disabled:pointer-events-none disabled:opacity-40"
          >
            <CheckCheck className="h-3.5 w-3.5" />
            Mark all read
          </button>
        </div>

        <ul className="max-h-96 overflow-y-auto">
          {notifications.length === 0 ? (
            <li className="px-4 py-10 text-center text-sm text-ink/50">
              You are all caught up.
            </li>
          ) : (
            notifications.slice(0, 10).map((item) => (
              <li key={item.id} className="border-b border-ink/[0.06] last:border-0">
                <button
                  type="button"
                  onClick={() => handleOpenNotification(item)}
                  className="flex w-full items-start gap-3 px-4 py-3 text-left hover:bg-ink/[0.04]"
                >
                  <span
                    className={cn(
                      "mt-1.5 h-2 w-2 shrink-0 rounded-full",
                      item.read ? "bg-ink/20" : "bg-brand-sky",
                    )}
                  />
                  <span className="min-w-0 flex-1">
                    <span className="block truncate text-sm font-medium">
                      {item.title}
                    </span>
                    <span className="mt-0.5 flex items-center gap-1.5 text-xs text-ink/50">
                      <span className="truncate font-mono">{item.topic}</span>
                      <span>·</span>
                      <RelativeTime date={item.timestamp} className="shrink-0" />
                    </span>
                  </span>
                </button>
              </li>
            ))
          )}
        </ul>

        <Link
          to="/requests"
          onClick={() => setOpen(false)}
          className="block border-t border-ink/10 px-4 py-2.5 text-center text-xs font-medium text-ink/70 hover:bg-ink/[0.04] hover:text-ink"
        >
          View all credential requests
        </Link>
      </PopoverContent>
    </Popover>
  );
}
