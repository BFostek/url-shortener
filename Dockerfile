# Stage 1: Building the application
FROM rust AS builder

WORKDIR /usr/src/urlshortener

COPY . .

RUN cargo build --release

# Stage 2: Setting up the runtime environment
FROM debian

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/urlshortener/target/release/urlshortener /usr/local/bin/urlshortener

# Copy the .env file from your source into the Docker image
COPY --from=builder /usr/src/urlshortener/.env /.env

CMD ["urlshortener"]

