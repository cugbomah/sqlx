# Start with a Rust build environment
FROM rust:1.69 as builder

# Create a new empty shell project
WORKDIR /usr/src/app

# First, copy over the manifest files (Cargo.toml and Cargo.lock)
# and the source code.
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# ENV DATABASE_URL=postgres://postgres:password123@localhost:5433/dtmw
# Build the release version of our project.
RUN cargo build --release

# Start a new stage for the runtime environment
FROM debian:bullseye-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/sqlx_example /usr/local/bin/

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/sqlx_example"]
