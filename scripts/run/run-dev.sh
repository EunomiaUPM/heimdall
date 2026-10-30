#!/bin/bash
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# ===============================
# 1. Environment variables
# ===============================
export VAULT_PATH="./vault/secrets"
export VAULT_APP_DB="db.json.example"
export VAULT_APP_WALLET="wallet.json.example"
export VAULT_APP_PRIV_KEY="private_key.json.example"
export VAULT_APP_PUB_PKEY="public_key.json.example"
export VAULT_APP_CERT="cert.json.example"
export RUST_BACKTRACE="1"
export VITE_GATEWAY_PORT="${VITE_GATEWAY_PORT:-1500}"
export VITE_PORT="${VITE_PORT:-5176}"

cleanup() {
    if [ -n "$FRONTEND_PID" ]; then
        echo -e "\n\033[0;33mStopping Vite front-end dev server (PID: $FRONTEND_PID)...\033[0m"
        kill "$FRONTEND_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT INT TERM

# ===============================
# 2. Start dependencies
# ===============================
echo -e "\033[0;36mStarting dependencies (heimdall)...\033[0m"
docker compose -f deployment/docker-compose.dev.yaml up -d

# ===============================
# 3. Wait for DB
# ===============================
echo -e "\033[0;36mWaiting for DB to be ready...\033[0m"
until docker exec heimdall-db pg_isready -U postgres >/dev/null 2>&1; do
    sleep 2
done
echo -e "\033[0;32mDB ready\033[0m"

# ===============================
# 4. Start React frontend (DEV)
# ===============================
echo -e "\033[0;36mStarting React frontend in DEV mode (port $VITE_PORT -> backend :$VITE_GATEWAY_PORT)...\033[0m"
(
    cd "$ROOT_DIR/react"
    npm install
    VITE_GATEWAY_PORT="$VITE_GATEWAY_PORT" VITE_PORT="$VITE_PORT" npm run dev
) &
FRONTEND_PID=$!
echo -e "\033[0;32mFrontend dev server spawned (http://localhost:$VITE_PORT/admin/)\033[0m"

# ===============================
# 5. Wait for fafnir-wallet
# ===============================
echo -e "\033[0;36mWaiting for fafnir-wallet to be ready...\033[0m"
WALLET_URL="http://localhost:7000/readiness"
until curl -fs "$WALLET_URL" >/dev/null 2>&1; do
    sleep 2
done
echo -e "\033[0;32mfafnir-wallet ready\033[0m"

# ===============================
# 6. Backend setup
# ===============================
echo -e "\033[0;36mRunning setup...\033[0m"
if ! cargo run setup -e ./static/config/dev/eco_authority.yaml; then
    echo -e "\033[0;31mSetup failed, aborting\033[0m"
    exit 1
fi

# ===============================
# 7. Start server
# ===============================
echo -e "\033[0;36mStarting Heimdall...\033[0m"
cargo watch -i "vault/*" -i "react/*" -x "run start -e ./static/config/dev/eco_authority.yaml"
