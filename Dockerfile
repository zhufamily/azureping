# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.92.0
ARG APP_NAME=azureping

################################################################################
# Build stage
FROM rust:${RUST_VERSION}-bookworm AS builder
ARG APP_NAME
WORKDIR /app

# Install system dependencies for Rust + C/C++ + OpenSSL
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    g++ \
    git \
    curl \
    ca-certificates \
    bash \
 && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Copy models folder
COPY models ./models

# Build the Rust project in release mode
RUN cargo build --release

################################################################################
# Final stage: minimal runtime
FROM ubuntu:24.04 AS final

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    bash \
    curl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary (NO trailing slash)
COPY --from=builder /app/target/release/azureping ./azureping

# Ensure executable bit (sometimes needed)
RUN chmod +x ./azureping

# Copy models
COPY --from=builder /app/models ./models

CMD ["./azureping"]
