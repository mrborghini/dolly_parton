# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies without the application code to speed up builds
RUN cargo build --release && \

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Set the startup command to run the compiled binary
CMD ["./target/release/dolly_parton"]
