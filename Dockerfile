# Stage 1: Build React Frontend
FROM node:20-slim AS frontend-builder
WORKDIR /app/react
COPY react/package*.json ./
RUN npm install --legacy-peer-deps
COPY react/ .
RUN npm run build

# Stage 2: Build Rust Backend
FROM rust:1.81-slim-bookworm AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src src
# COPY .git .git # Optional: copy git info if needed for versioning
RUN cargo build --release

# Stage 3: Final Runtime Image
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies (OpenSSL)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/heimdall /app/heimdall

# Copy frontend build output to expected path
COPY --from=frontend-builder /app/react/dist /app/react/dist

# Copy static configuration and environment files
COPY static /app/static

# Expose port
EXPOSE 1500

# Set environment variable for log level (optional default)
ENV RUST_LOG=info

# Command to run (using the config from volume or image)
# We assume the env file path is passed via arguments or default
ENTRYPOINT ["/app/heimdall"]
CMD ["start", "--env-file", "/app/static/environment/envs/.env"]
