# Build stage
FROM rust:1.72-slim as builder

WORKDIR /usr/src/app

# Copy the Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/notion-crud /usr/local/bin/notion-crud

# Expose port 80
EXPOSE 80

# Set environment variables
ENV RUST_LOG=info
ENV PORT=80

# Run the application
CMD ["sh", "-c", "notion-crud"] 