#!/bin/bash
set -e

# ===============================
# 1️⃣ Variables de entorno
# ===============================
export VAULT_PATH="./vault/secrets"
export VAULT_APP_DB="db.json.example"
export VAULT_APP_WALLET="wallet.json.example"
export VAULT_APP_PRIV_KEY="private_key.json.example"
export VAULT_APP_PUB_PKEY="public_key.json.example"
export VAULT_APP_CERT="cert.json.example"
export RUST_BACKTRACE="full"

# ===============================
# 2️⃣ Levantar dependencias
# ===============================
echo -e "\033[0;36mLevantando dependencias...\033[0m"
docker compose -f docker-compose.dev.yml up -d

# ===============================
# 3️⃣ Esperar a que la DB esté lista
# ===============================
echo -e "\033[0;36mEsperando a que la DB esté lista...\033[0m"
until docker exec heimdall-db pg_isready -U postgres > /dev/null 2>&1; do
    sleep 2
done
echo -e "\033[0;32mDB lista\033[0m"

# ===============================
# 4️⃣ Build React
# ===============================
echo -e "\033[0;36mPreparando y construyendo la app React...\033[0m"
chmod +x ./react/build.sh
cd ./react
./build.sh
if [ $? -ne 0 ]; then
    echo -e "\033[0;31mBuild de React fallido, abortando\033[0m"
    exit 1
fi


# ===============================
# 4️⃣.5️⃣ Setup Heimdall
# ===============================
cd ..
echo -e "\033[0;36mEjecutando setup...\033[0m"
cargo run setup -e ./static/environment/config/dev/basic_dataspace_authority.yaml
if [ $? -ne 0 ]; then
    echo -e "\033[0;31mSetup fallido, abortando\033[0m"
    exit 1
fi


# ===============================
# 5️⃣ Start Heimdall
# ===============================
echo -e "\033[0;36mArrancando Heimdall...\033[0m"
cargo watch -x "run start -e ./static/environment/config/dev/basic_dataspace_authority.yaml"