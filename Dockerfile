# Multi-stage build for minimal image size
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY bootstrap-macros ./bootstrap-macros
COPY telemetry-macros ./telemetry-macros

# Copy source
COPY *.rs ./

# Build release binaries
RUN cargo build --release --bins

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /build/target/release/minimal-build-server /usr/local/bin/
COPY --from=builder /build/target/release/demo_* /usr/local/bin/

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /usr/local/bin

USER appuser
WORKDIR /home/appuser

EXPOSE 8080

CMD ["minimal-build-server"]
