import React, { createContext, useContext, useEffect, useState } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';

const NotificationContext = createContext();

export const useNotifications = () => {
  return useContext(NotificationContext);
};

export default function NotificationProvider({ children }) {
  const [notifications, setNotifications] = useState([]);
  const [unreadCount, setUnreadCount] = useState(0);

  useEffect(() => {
    // Inject a local notification if the wallet is not yet linked
    if (localStorage.getItem('walletOnboarded') !== 'true') {
      setNotifications((prev) => {
        if (!prev.some((n) => n.id === 'wallet-unlink-warning')) {
          setUnreadCount((c) => c + 1);
          return [
            {
              id: 'wallet-unlink-warning',
              title: 'Action Required: Link Wallet',
              message: 'Your wallet is not linked yet. Click here to initialize it and enable full Heimdall functionality.',
              created_at: new Date().toISOString(),
              link: '/wallet?autolink=true',
            },
            ...prev,
          ];
        }
        return prev;
      });
    }

    const sseUrl = `${apiUrl}/react/notifications/stream`;
    console.log('Connecting to SSE at', sseUrl);

    const eventSource = new EventSource(sseUrl);

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log('New notification:', data);
        setNotifications((prev) => [data, ...prev]);
        setUnreadCount((prev) => prev + 1);
      } catch (err) {
        console.error('Error parsing notification data:', err);
      }
    };

    eventSource.onerror = (error) => {
      console.error('SSE connection error:', error);
    };

    return () => {
      eventSource.close();
    };
  }, []);

  const clearNotifications = () => {
    setNotifications([]);
    setUnreadCount(0);
  };

  const markAllAsRead = () => {
    setUnreadCount(0);
  };

  return (
    <NotificationContext.Provider
      value={{ notifications, unreadCount, clearNotifications, markAllAsRead }}
    >
      {children}
    </NotificationContext.Provider>
  );
}
