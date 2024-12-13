# Use the official Rust image as a builder
FROM rust:1.74 as builder

# Create and set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock to the container
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN cargo build --release

# Now copy the entire source code to the container
COPY . .

# Build the final executable
RUN cargo build --release

# Use a minimal base image to run the application
FROM debian:buster-slim

# Install the necessary dependencies (like openssl for axum)
RUN apt-get update && apt-get install -y openssl

# Copy the compiled executable from the builder stage
COPY --from=builder /app/target/release/my_service /usr/local/bin/auth-service

# Set environment variables
ENV RUST_LOG=info

# Expose the application port
EXPOSE 3000

# Start the application
CMD ["auth-service"]
