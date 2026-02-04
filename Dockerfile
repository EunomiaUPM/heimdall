# -------- build --------
FROM rust:alpine3.23 AS builder

WORKDIR /build

# mejor cache
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src ./src
COPY static ./static

RUN cargo build --release


# -------- runtime --------
FROM debian:bookworm-slim

WORKDIR /app

# instalar CA certificates (necesario para reqwest/rustls)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# binario linux
COPY --from=builder /build/target/release/heimdall /app/heimdall

# config
COPY static/environment/config/dataspace_authority.yaml /app/dataspace.yaml

# secretos dummy (SÍ, dentro)
COPY vault/secrets /vault/secrets
COPY static /static

ENV VAULT_PATH=/vault/secrets

EXPOSE 1500

CMD ["sh", "-c", "/app/heimdall setup -e /app/dataspace.yaml && /app/heimdall start -e /app/dataspace.yaml"]
