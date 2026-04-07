// Re-export from AuthContext so existing imports keep working.
// The actual logic and cache live in AuthContext (fetched once for the whole app).
export { useAuth } from '@/contexts/AuthContext';

