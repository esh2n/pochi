# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080
ENV SECRET_TOKEN=${SECRET_TOKEN}

# Run the web service on container startup.
ENTRYPOINT ["target/release/pochi"]

EXPOSE 8080