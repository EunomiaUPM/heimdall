import React, { createContext, useCallback, useContext, useEffect, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { getApiGatewayBase } from "@eunomia/ui";

export interface HeimdallNotification {
  id: string;
  title: string;
  topic: string;
  timestamp: string;
  requestId?: string;
  participantId?: string;
  read: boolean;
  raw: unknown;
}

interface NotificationContextValue {
  notifications: HeimdallNotification[];
  unreadCount: number;
  connected: boolean;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  clearNotifications: () => void;
}

const NotificationContext = createContext<NotificationContextValue | null>(null);

function normalizeEvent(raw: unknown): HeimdallNotification {
  const now = new Date().toISOString();
  const obj = typeof raw === "object" && raw !== null ? (raw as Record<string, unknown>) : {};
  const id =
    (typeof obj.id === "string" && obj.id) ||
    `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  const topic =
    (typeof obj.topic === "string" && obj.topic) ||
    (typeof obj.event === "string" && obj.event) ||
    (typeof obj.kind === "string" && obj.kind) ||
    "authority.event";
  const title =
    (typeof obj.message === "string" && obj.message) ||
    (typeof obj.title === "string" && obj.title) ||
    (typeof obj.status === "string" ? `Request ${obj.status}` : "New authority notification");
  const timestamp =
    (typeof obj.timestamp === "string" && obj.timestamp) ||
    (typeof obj.created_at === "string" && obj.created_at) ||
    now;
  const requestId =
    (typeof obj.request_id === "string" && obj.request_id) ||
    (typeof obj.grant_id === "string" && obj.grant_id) ||
    (typeof obj.id === "string" ? obj.id : undefined);
  const participantId =
    typeof obj.participant_id === "string" ? obj.participant_id : undefined;

  return {
    id,
    title,
    topic,
    timestamp,
    requestId,
    participantId,
    read: false,
    raw,
  };
}

export function NotificationProvider({ children }: { children: React.ReactNode }) {
  const queryClient = useQueryClient();
  const [notifications, setNotifications] = useState<HeimdallNotification[]>([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const apiBase = getApiGatewayBase() || import.meta.env.VITE_API_PATH || "/api/v1";
    const sseUrl = `${apiBase}/react/notifications/stream`;
    const eventSource = new EventSource(sseUrl, { withCredentials: true });

    eventSource.onopen = () => {
      setConnected(true);
    };

    eventSource.onmessage = (event) => {
      try {
        const parsed = JSON.parse(event.data);
        const item = normalizeEvent(parsed);
        setNotifications((prev) => [item, ...prev].slice(0, 50));
        // Refresh Approver & Participants queries automatically on live SSE events
        void queryClient.invalidateQueries({
          predicate: (q) =>
            typeof q.queryKey[0] === "string" &&
            (q.queryKey[0].startsWith("/approver") || q.queryKey[0].startsWith("/minions")),
        });
      } catch {
        // Ignore non-JSON keepalive comments
      }
    };

    eventSource.onerror = () => {
      setConnected(false);
    };

    return () => {
      eventSource.close();
    };
  }, [queryClient]);

  const markAsRead = useCallback((id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n)),
    );
  }, []);

  const markAllAsRead = useCallback(() => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  }, []);

  const clearNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  const unreadCount = notifications.reduce((acc, n) => acc + (n.read ? 0 : 1), 0);

  return (
    <NotificationContext.Provider
      value={{
        notifications,
        unreadCount,
        connected,
        markAsRead,
        markAllAsRead,
        clearNotifications,
      }}
    >
      {children}
    </NotificationContext.Provider>
  );
}

export function useNotifications(): NotificationContextValue {
  const ctx = useContext(NotificationContext);
  if (!ctx) {
    throw new Error("useNotifications must be used within a NotificationProvider");
  }
  return ctx;
}
