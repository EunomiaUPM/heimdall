# 🛡️ Heimdall

Heimdall is a comprehensive **Self-Sovereign Identity (SSI) Authority** and **Wallet Manager**. It acts as a central pillar in an identity ecosystem, facilitating the issuance, verification, and management of Verifiable Credentials (VCs) and Verifiable Presentations (VPs).

## 🌍 Overview

Heimdall provides a modular architecture to handle digital identity:

- **Issuer** 📜: Issues Verifiable Credentials (OID4VCI).
- **Verifier** ✅: Verifies Verifiable Presentations (OIDV4VP).
- **GateKeeper** 🔑: Manages fine-grained authorization (GNAP).
- **Approver** ⚖️: Handles credential approval workflows.
- **Wallet** 💼: Embedded wallet for keys and DIDs.
- **Web Interface** 🖥️: A built-in React dashboard for management.

Built with a **Clean Architecture** approach 🏗️, ensuring robustness and maintainability.

---

## 🏗️ Architecture

_(Espacio reservado para añadir una foto/diagrama de la arquitectura)_
![Heimdall Architecture](path/to/architecture_image.png)

---

## 🧩 Modules

Heimdall consists of several functional modules:

- **Gatekeeper** 🔑: GNAP-based authorization service for advanced access control.
- **Issuer** 📜: OID4VCI compliant service for issuing Verifiable Credentials.
- **Verifier** ✅: OID4VP compliant service for validating Verifiable Presentations.
- **Minion Manager** 👥: Handles participant management and entity onboarding.
- **Approver** ⚖️: Manages credential approval workflows and policies.
- **Wallet** 💼: Secure management of cryptographic keys and Decentralized Identifiers (DIDs).
- **Admin UI** 🖥️: A React-based management dashboard accessible at `/admin/home`.

---

## 🎭 Roles

Heimdall supports different authority roles, each specialized in issuing specific sets of credentials:

- **Legal Authority**: Focused on legal identity. Issues `EORI`, `LEI`, `VAT ID`, `Tax ID`, and local registration numbers.
- **Dataspace Authority**: Specialized in ecosystem participation. Issues `DataspaceParticipant` credentials.
- **Clearing House**: Infrastructure role for clearing and settlement logic. (Not implemented yet)
- **Clearing House Proxy**: Proxy service for clearing house interactions. (Not implemented yet)
- **Eco Authority**: A comprehensive hybrid role that combines all role capabilities.

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

- `static/environment/config/....yaml`
- `static/environment/envs/....env`
- `static/specs`: `openapi.json`.

---

## 📚 API Documentation

A complete **OpenAPI 3.1.0** specification is available.

- **File**: `static/specs/openapi/openapi.json` 📄
- **Online**: Access `/api/v1/docs` when running locally via `openapi_router.rs`.

---

## 🌍 Deployment

Heimdall can be deployed in different environments. Below are the primary deployment methods:

### 1. Mini Deployment (Local / Quickstart)

The `mini` deployment is the fastest way to spin up the Heimdall core and its database using Docker Compose. It leverages `docker-compose.yml`.

**Steps to deploy:**

1. Clone the repository.
2. Ensure you have Docker and Docker Compose installed.
3. You can configure your environment variables if you want, but this version runs perfectly with the .example
4. Run the following command:
   ```bash
   docker-compose up
   ```

This will start:

- `heimdall-db`: The PostgreSQL persistence layer on port `1450`.
- `heimdall`: The main backend application serving on port `1500`, connected to the database.

### 2. Prod Deployment (Production)

For a full production deployment with Vault, Keycloak, and oauth2-proxy, see the [Production Deployment Guide](./PROD-DEPLOYMENT.md).

## 🛠️ Development

### Prerequisites

- 🦀 **Rust** (latest stable)
- 🟢 **Node.js & npm** (for the React frontend)
- 🗄️ **Database** (configured via `DatabaseConfig`)

### Building & Running (Full Stack)

To run the complete system (Rust Backend + React Frontend), follow these steps:

1. **Build the Frontend**:
   Before running Rust, you must compile the React application code.

   **Windows (PowerShell):**

   ```powershell
   .\react\build.ps1
   ```

   **Linux/Mac (Bash):**

   ```bash
   ./react/build.sh
   ```

   _This generates the `react/dist` folder._

2. **Run the Server**:

   ```bash
   docker compose -f docker-compose.dev.yml up -d
   ```

   Depending on yor sistem copy the file heimdall.env.ps1.example o heimdall.env.sh.example into your terminal so that heimdall process can capture those environment variables

   ```bash
   cargo run setup -e ./static/environment/config/dev/dev_basic_dataspaces_authority.yaml
   cargo run watch -x "run start -e ./static/environment/config/dev/dev_basic_dataspaces_authority.yaml"
   ```

   The server will start (default port 1500) and serve the frontend at:
   `http://localhost:1500/admin/home`

### Development Mode

- **Rust**: `cargo run` (Backend API only)
- **React**: `cd react && npm run dev` (Frontend with hot-reload at `localhost:5173`)
  _Note: In this mode, the frontend runs separately from the Rust server backend._

---

## 📄 License

Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
GNU General Public License v3.0
