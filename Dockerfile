# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /App

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY . ./

# Build the application
RUN cargo build --release

# Set the startup command to run the compiled binary
CMD ["./target/release/dolly_parton"]
