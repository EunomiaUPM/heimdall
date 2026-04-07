// The backend serves both the API and the frontend at the same origin,
// so a relative path works in both dev (localhost:1500) and prod (behind oauth2-proxy).
// VITE_API_PATH is kept in .env in case it ever needs to change, but VITE_HOST_URL is gone.
export const VITE_API_SERVER_URL = import.meta.env.VITE_API_PATH ?? '/api/v1';
