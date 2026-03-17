import { createContext, useContext, useState, useEffect } from 'react';

const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  const authEnabled = import.meta.env.VITE_AUTH_ENABLED === 'true';

  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(authEnabled);

  useEffect(() => {
    if (!authEnabled) return;

    fetch('/oauth2/userinfo', { credentials: 'include' })
      .then((res) => (res.ok ? res.json() : null))
      .then((data) => { if (data) setUser(data); })
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []); // runs once on mount

  const logout = () => {
    if (!authEnabled) return;
    setUser(null); // clear cache immediately
    window.location.href = '/oauth2/sign_out';
  };

  return (
    <AuthContext.Provider value={{ user, loading, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used inside <AuthProvider>');
  return ctx;
}
