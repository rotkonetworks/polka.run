# Start with the Rust nightly image
FROM rustlang/rust:nightly-bookworm-slim

# Install Node.js, OpenSSL development libraries, and pkg-config
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    gnupg \
    libssl-dev \
    pkg-config \
    nodejs
