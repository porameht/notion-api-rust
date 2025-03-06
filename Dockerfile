# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy only Cargo.toml first (since Cargo.lock may not exist)
COPY Cargo.toml ./

# Create dummy src structure with all required files
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/create_database.rs && \
    # Build dependencies only
    cargo build && \
    # Remove the dummy source files, but keep the generated artifacts
    find src -type f -name "*.rs" -delete

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