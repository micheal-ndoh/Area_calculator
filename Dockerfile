FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/area

COPY . .

# Build the Rust program
RUN cargo build --release


FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/src/area

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/cpu/target/release/area .

# Run the executable
CMD ["./area"]
