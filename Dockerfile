FROM rust:1.75-slim as builder

WORKDIR /usr/src/orvrm

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install dependencies for OSRM client
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/orvrm/target/release/orvrm /app/orvrm
COPY --from=builder /usr/src/orvrm/config /app/config

# Expose the API port
EXPOSE 8080

# Run the binary
CMD ["/app/orvrm"] 