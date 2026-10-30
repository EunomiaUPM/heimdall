# Scripts

Helper scripts, grouped by purpose. Every script resolves its paths from its own location, so it
can be launched from any directory.

| Folder | Scripts | What they do |
|---|---|---|
| `run/` | `run-mini.sh` | Brings up the mini deployment (`deployment/docker-compose.mini.yaml`: database, Fafnir wallet and Heimdall) and waits until Heimdall answers on `:1500`. `--down` to stop. |
| | `run-dev.sh` | Dev mode (like `ds-protocol`): infrastructure in Docker (`deployment/docker-compose.dev.yaml`), React Vite HMR dev server on `:5176` (`/admin/`), and Heimdall from source with `cargo watch` on `:1500`. |
| | `run-react-dev.sh` | Launches only the React Vite HMR dev server (`http://localhost:5176/admin/`) proxying `/api`, `/oauth2` and `/.well-known` to `:1500`. |
| | `compile.sh` | Builds the production TypeScript bundle into `react/dist`, which Heimdall serves at `/admin/`. |
