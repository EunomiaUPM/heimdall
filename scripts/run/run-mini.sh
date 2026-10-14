#!/bin/bash
# Brings up heimdall's mini stack (published images) and waits until it answers.
# Usage: scripts/run/run-mini.sh [--down]
set -e

COMPOSE_FILE="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../deployment" && pwd)/docker-compose.mini.yaml"
PORT=1500

DOWN=false
for arg in "$@"; do
    case "$arg" in
        --down|-d) DOWN=true ;;
        *) echo "Uso: $0 [--down]" >&2; exit 2 ;;
    esac
done

# Any answer below 500 counts as alive; gives up after $2 seconds (180 by default).
wait_http() {
    local url="$1" timeout="${2:-180}" deadline code
    deadline=$(( $(date +%s) + timeout ))
    while [ "$(date +%s)" -lt "$deadline" ]; do
        code=$(curl -s -o /dev/null -m 2 -w "%{http_code}" "$url" || true)
        if [ "$code" != "000" ] && [ "$code" -lt 500 ]; then
            return 0
        fi
        sleep 2
    done
    return 1
}

start_heimdall() {
    echo -e "\033[0;36mLevantando heimdall (mini)...\033[0m"
    if ! docker compose -f "$COMPOSE_FILE" up -d; then
        echo -e "\033[0;31mFallo al levantar heimdall. Logs del setup: docker logs heimdall-setup\033[0m"
        exit 1
    fi

    echo -e "\033[0;36mEsperando a que heimdall responda en el puerto $PORT...\033[0m"
    if ! wait_http "http://localhost:$PORT/readiness"; then
        echo -e "\033[0;31mheimdall no responde. Logs: docker logs heimdall\033[0m"
        exit 1
    fi
    echo -e "\033[0;32mheimdall listo: http://localhost:$PORT/admin/\033[0m"
}

stop_heimdall() {
    echo -e "\033[0;36mBajando heimdall...\033[0m"
    if ! docker compose -f "$COMPOSE_FILE" down; then
        echo -e "\033[0;31mFallo al bajar heimdall\033[0m"
        exit 1
    fi
}

if [ "$DOWN" = true ]; then stop_heimdall; else start_heimdall; fi
