# Build stage
FROM rust:1.81-slim as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    & rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    & rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

CMD ["backend"]
