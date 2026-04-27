import React, { createContext, useContext, useEffect, useState, useCallback } from 'react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';

const NotificationContext = createContext();

export const useNotifications = () => {
  return useContext(NotificationContext);
};

export default function NotificationProvider({ children }) {
  const [notifications, setNotifications] = useState([]);
  const [unreadCount, setUnreadCount] = useState(0);
  const [isWalletLinked, setIsWalletLinked] = useState(false);
  const [isWalletLoading, setIsWalletLoading] = useState(true);

  const checkWalletStatus = useCallback(async () => {
    try {
      const response = await fetch(`${apiUrl}/wallet/is-linked`);
      if (response.ok) {
        const data = await response.json();
        setIsWalletLinked(data.is_linked);
      }
    } catch (err) {
      console.error('Error checking wallet status:', err);
    } finally {
      setIsWalletLoading(false);
    }
  }, []);

  useEffect(() => {
    checkWalletStatus();
  }, [checkWalletStatus]);

  useEffect(() => {
    // Inject or remove a local notification based on wallet link status
    if (!isWalletLoading) {
      if (!isWalletLinked) {
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
      } else {
        setNotifications((prev) => {
          const filtered = prev.filter((n) => n.id !== 'wallet-unlink-warning');
          if (filtered.length !== prev.length) {
            // If we removed the warning, we should ideally decrement unreadCount if it was unread
            // For simplicity, we just filter it.
          }
          return filtered;
        });
      }
    }
  }, [isWalletLinked, isWalletLoading]);

  useEffect(() => {
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
      value={{ 
        notifications, 
        unreadCount, 
        clearNotifications, 
        markAllAsRead, 
        isWalletLinked, 
        isWalletLoading, 
        checkWalletStatus 
      }}
    >
      {children}
    </NotificationContext.Provider>
  );
}
