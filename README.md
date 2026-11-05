# Heimdall

The authority of an [EUNOMIA](https://eunomia.dit.upm.es) data space. Heimdall onboards participants: it decides credential requests over GNAP, verifies presentations over OID4VP and issues verifiable credentials over OID4VCI.

**Documentation: https://eunomiaupm.github.io/heimdall**

This README describes `v1.5.1`. Its image is still `quay.io/eunomia_upm/heimdall:v1.5.0`: this tag only adds documentation. The Docker Hub image `rmenendez8/heimdall:v2.1.0`, used by the Compose files, is the same image.

## Quickstart

You need Docker with Compose.

```bash
git clone --branch v1.5.1 https://github.com/EunomiaUPM/heimdall.git
cd heimdall
docker compose up -d
curl -X POST http://127.0.0.1:1500/api/v1/wallet/link
curl http://127.0.0.1:1500/.well-known/did.json
```

This runs an `EcoAuthority` with its PostgreSQL and its Fafnir wallet. The console is at `http://127.0.0.1:1500/admin`.

The example keys and passwords are public: use them only on your machine.

## What it does

| Module | Path | Purpose |
| --- | --- | --- |
| Gatekeeper | `/api/v1/gate` | GNAP grants, signed with HTTP Message Signatures |
| Verifier | `/api/v1/verifier` | OID4VP presentations |
| Issuer | `/api/v1/issuer` | OID4VCI issuance |
| Approver | `/api/v1/approver` | The credential requests and their state |
| Participants | `/api/v1/minions` | The participants it knows |
| Wallet | `/api/v1/wallet` | Its keys, DIDs and credentials, through Fafnir |
| Console | `/admin` | React interface for operators |

## Roles

`role` in the configuration sets what the authority issues:

| Role | Issues |
| --- | --- |
| `DataSpaceAuthority` | `DataSpaceParticipant` |
| `LegalAuthority` | `gx:VatId`, `gx:TaxId`, `gx:LeiCode`, `gx:LocalRegistrationNumber` |
| `ClearingHouse` | `gx:LabelCredential` |
| `EcoAuthority` | All of the above |

`ClearingHouse` imitates the behaviour of a Gaia-X Digital Clearing House: it checks the credentials a holder presents and issues a label. The `gx:` names follow the Gaia-X vocabulary, but this authority signs these credentials itself. `v1.5.1` does not claim conformance with Gaia-X.

## Configuration

An authority reads one YAML file, passed to `heimdall setup` and `heimdall start` with `--env-file`. Examples are in [`static/config`](static/config), and the [configuration guide](https://eunomiaupm.github.io/heimdall/docs/configuration) explains each field.

## Production

[`PROD-DEPLOYMENT.md`](PROD-DEPLOYMENT.md) covers a deployment with HashiCorp Vault, Keycloak and oauth2-proxy.

## API

The [API documentation](https://eunomiaupm.github.io/heimdall/docs/api) lists the routes of this release. The spec is in [`static/specs/openapi/openapi.json`](static/specs/openapi/openapi.json), and a running authority serves it at `/api/v1/docs/openapi`.

## Development

You need Rust, Node.js and Docker.

```bash
./start-dev.sh
```

The script starts the dependencies, builds the console, runs `setup` and starts the server with `cargo watch`, using `static/config/dev/eco_authority.yaml`. To work on the console with hot reload:

```bash
cd react
npm run dev
```

The documentation site is in [`docs`](docs).

## What's coming

The next version adds an event bus with webhooks, a new console and a new credential, and it changes some configuration keys. Further ahead are the real integration with Gaia-X and DCP. See [What's coming](https://eunomiaupm.github.io/heimdall/docs/whats-coming).

## License

Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM

GNU General Public License v3.0. See [`LICENSE`](LICENSE).
