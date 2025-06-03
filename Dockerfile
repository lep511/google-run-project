# Use the official Rust image as a parent image
FROM rust:1.87-slim as builder

# Install system dependencies required for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Set the working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/ai-chat-app /app/

# Copy static files (HTML, CSS, JS)
COPY static ./static

# Change ownership to appuser
RUN chown -R appuser:appuser /app
USER appuser

# Expose the port that the application listens on
EXPOSE 8080

# Set environment variables for Google Cloud Run
ENV PORT=8080
ENV RUST_LOG=info

# Health check (optional but recommended for Google Cloud Run)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/ || exit 1

# Run the web service on container startup
CMD ["./ai-chat-app"]