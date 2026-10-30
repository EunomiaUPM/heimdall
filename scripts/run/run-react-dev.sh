#!/bin/bash
# Starts the Heimdall React Admin GUI in Vite HMR dev mode against a running Heimdall backend (:1500).
# Usage: scripts/run/run-react-dev.sh
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/react"

export VITE_GATEWAY_PORT="${VITE_GATEWAY_PORT:-1500}"
export VITE_PORT="${VITE_PORT:-5176}"

echo -e "\033[0;36mInstalling frontend dependencies...\033[0m"
npm install

echo -e "\033[0;32mStarting Heimdall React dev server on http://localhost:$VITE_PORT/admin/ (proxying API to :$VITE_GATEWAY_PORT)...\033[0m"
exec npm run dev
