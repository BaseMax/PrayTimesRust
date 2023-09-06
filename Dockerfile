# Use the rust image as the base
FROM rust:1.70-slim-buster as build
# Set the working directory
WORKDIR /app
# Copy the whole workspace
COPY ./ ./

RUN cargo build --release --workspace
# Build only the praytimes-kit binary

# # Start a new stage to reduce image size
FROM debian:buster-slim

# Copy the binary from the build stage
COPY --from=build /app/target/release/praytimes-kit /usr/local/bin

# Make the binary executable
RUN chmod +x /usr/local/bin/praytimes-kit

Env PRAYTIMES_LOG=info
# Set the binary as the entrypoint
ENTRYPOINT ["praytimes-kit"]

CMD ["serve"]
