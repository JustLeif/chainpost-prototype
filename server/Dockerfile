# Build Stage
FROM rust:latest as builder
WORKDIR /usr/src/chainpost-server
COPY . .
RUN cargo build --release

# Runtime Stage
FROM debian:latest
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY static /app/static
# Copy the binary from the builder stage
COPY --from=builder /usr/src/chainpost-server/target/release/chainpost-server /app/chainpost-server

# Expose port 8080
EXPOSE 8080

# Command to run the binary
CMD ["./chainpost-server"]