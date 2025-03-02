# Use the official Rust image as a base
FROM rust:1.85 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo files first (to leverage Docker cache)
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to resolve dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Copy the actual source code
Copy src/ src/

# Build the application
RUN cargo build --release

# Create a lightweight final image
FROM debian:bookworm-slim

# Copy the Rust binary from the builder stage
COPY --from=builder /app/target/release/news-topic-search-rust-server /usr/local/bin/news-topic-search-rust-server

# Set the binary as the entry point
CMD ["news-topic-search-rust-server"]
