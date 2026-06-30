#!/bin/bash
set -e

# ===============================
# 1. Variables de entorno
# ===============================

export VAULT_PATH="./vault/secrets"
export VAULT_APP_DB="db.json.example"
export VAULT_APP_WALLET="wallet.json.example"
export VAULT_APP_PRIV_KEY="private_key.json.example"
export VAULT_APP_PUB_PKEY="public_key.json.example"
export VAULT_APP_CERT="cert.json.example"
export RUST_BACKTRACE="full"

# ===============================
# 2. Levantar dependencias
# ===============================

echo -e "\033[0;36mLevantando dependencias...\033[0m"
docker compose -f docker-compose.dev.yml up -d

# Si quieres compatibilidad con Docker Compose antiguo:
# docker-compose -f docker-compose.dev.yml up -d

# ===============================
# 3. Esperar a que la DB esté lista
# ===============================

echo -e "\033[0;36mEsperando a que la DB esté lista...\033[0m"

until docker exec heimdall-db pg_isready -U postgres >/dev/null 2>&1; do
    sleep 2
done

echo -e "\033[0;32mDB lista\033[0m"

# ===============================
# 4. Build React
# ===============================

echo -e "\033[0;36mPreparando y construyendo la app React...\033[0m"

chmod +x ./react/build.sh

(
    cd ./react
    ./build.sh
)

echo -e "\033[0;32mBuild de React completado\033[0m"

# ===============================
# 5. Setup
# ===============================

echo -e "\033[0;36mEjecutando setup...\033[0m"

cargo run setup -e ./static/config/dev/eco_authority.yaml

# ===============================
# 6. Start
# ===============================

echo -e "\033[0;36mArrancando Heimdall...\033[0m"

cargo watch -i "vault/*" -x "run start -e ./static/config/dev/eco_authority.yaml"