# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Create a new binary project
COPY . .

# Build the application
RUN cargo build --release

# Test stage
FROM rust:latest

WORKDIR /app
COPY --from=builder /app .

# Default command to run tests
CMD ["cargo", "test"]



