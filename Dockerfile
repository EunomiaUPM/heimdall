# Stage 1: Build React Frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app/react
COPY react/package*.json ./
RUN npm install --legacy-peer-deps
COPY react/ .
RUN npm run build

# Stage 2: Build Rust Backend
FROM rust:alpine3.23 AS backend-builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

# Stage 3: Final Runtime Image
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies (OpenSSL)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/heimdall /app/heimdall

# Copy frontend build output
COPY --from=frontend-builder /app/react/dist /app/react/dist

# Copy static configuration
COPY static /app/static

EXPOSE 1500

ENV RUST_LOG=info
ENV CONFIG_PATH=/app/static/environment/config/prod/dataspace_authority.yaml

# Ejecutar setup + start
ENTRYPOINT ["/bin/sh", "-c"]
CMD ["/app/heimdall setup --env-file $CONFIG_PATH && /app/heimdall start --env-file $CONFIG_PATH"]
