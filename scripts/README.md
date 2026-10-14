# Scripts

Helper scripts, grouped by purpose. Every script resolves its paths from its own location, so it
can be launched from any directory.

| Folder | Scripts | What they do |
|---|---|---|
| `run/` | `run-mini.ps1` | Brings up the mini deployment (`docker-compose.yml`: database, Fafnir wallet and Heimdall) and waits until Heimdall answers on `:1500`. `-Down` to stop. |
| | `start-dev.ps1`, `start-dev.sh` | Dev mode: infrastructure in Docker (`docker-compose.dev.yml`), Heimdall from source with `cargo watch`. |
| | `start-dev-dev.ps1` | Older variant of `start-dev.ps1`; it points to a config that no longer exists. |

The scripts of the admin console (`build`, `start`) live with it, in `react/`.
