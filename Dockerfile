FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/area

COPY . .

# Build the Rust program
RUN cargo build --release

FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/area

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/area/target/release/area .

# Run the executable
CMD ["./area"]
