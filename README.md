# 🛡️ Heimdall

Heimdall is a comprehensive **Self-Sovereign Identity (SSI) Authority** and **Wallet Manager**. It acts as a central pillar in an identity ecosystem, facilitating the issuance, verification, and management of Verifiable Credentials (VCs) and Verifiable Presentations (VPs).

## 🌍 Overview

Heimdall provides a modular architecture to handle digital identity:

- **Issuer** 📜: Issues Verifiable Credentials (OID4VCI).
- **Verifier** ✅: Verifies Verifiable Presentations.
- **GateKeeper** 🔑: Manages fine-grained authorization (GNAP).
- **Approver** ⚖️: Handles credential approval workflows.
- **Wallet** 💼: Embedded wallet for keys and DIDs.

Built with a **Clean Architecture** approach 🏗️, ensuring robustness and maintainability.

---

## 🧩 Modules

### 🔑 1. GateKeeper (GNAP)

Replaces traditional OAuth2 for advanced scenarios.

- **Endpoints**: `/api/v1/gate/access`, `/api/v1/gate/continue/{id}`
- **Role**: Validates requests, issues access tokens.

### 📜 2. Issuer (OID4VCI)

Compliant with OpenID4VCI standards.

- **Endpoints**: `/.well-known/...`, `/credential`, `/token`

### ✅ 3. Verifier

Validates proofs provided by holders.

- **Endpoints**: `/api/v1/verifier/pd/{state}`, `/api/v1/verifier/verify/{state}`

### 💼 4. Wallet

Manages cryptographic keys (EdDSA, RSA) and DIDs (`did:web`, `did:key`).

---

## ⚙️ Configuration

Heimdall loads configuration from the `static/environment` directory 📂.

### 📝 Structure

The `CoreApplicationConfig` (`src/config/config.rs`) aggregates:

- **Host** 🏠: Server settings.
- **Database** 🗄️: persistence layer.
- **API** 🌐: Versioning and specs.
- **Role** 🎭: Deployment role (Issuer/Verifier/etc.).

### 📂 Files

- `static/environment`: `config.yaml` / `config.json`.
- `static/specs`: `openapi.json`.

---

## 🚀 Initialization & Startup

The initialization logic is handled in `src/setup`, primarily via the `AuthorityApplication` struct.

### 🔄 Startup Process

1.  **Configuration Load**: Reads the active config file.
2.  **Vault Setup** 🔐: `VaultService` is initialized to handle secrets and database connections safely.
3.  **Service Assembly**:
    - Creates necessary services based on the active **Role**.
    - Initializes **GitHub/Postgres** repositories.
    - Sets up **GNAP**, **Issuer**, and **Verifier** services.
4.  **Core Creation**: Assembles all services into the `Core` struct.
5.  **Router**: Builds the Axum router with all module routes.

### 🔒 TLS & Fallback

Heimdall attempts to start with **TLS** enabled by default (`run_tls`):

- Reads certificates/keys from environment variables (`VAULT_CLIENT_CERT`, `VAULT_CLIENT_KEY`).
- If TLS fails (e.g., missing certs in local dev), it automatically falls back to a basic HTTP server (`run_basic`).

---

## 📚 API Documentation

A complete **OpenAPI 3.1.0** specification is available.

- **File**: `static/specs/openapi/openapi.json` 📄
- **Online**: Access `/api/v1/docs` when running locally via `openapi_router.rs`.

---

## 🛠️ Development

### Prerequisites

- 🦀 Rust (latest stable)
- 🗄️ Database (configured via `DatabaseConfig`)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

---

## 📄 License

Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
GNU General Public License v3.0
