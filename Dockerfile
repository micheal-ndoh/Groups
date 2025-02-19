FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/Groups

COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the Rust program
RUN cargo build --release

FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/Groups

COPY --from=builder /usr/src/Groups/target/release/Groups .

# Run the executable
CMD ["./Groups"]