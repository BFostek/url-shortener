FROM rust AS builder

WORKDIR /usr/src/urlshortener

COPY . .

RUN cargo build --release

FROM debian

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/urlshortener/target/release/urlshortener /usr/local/bin/urlshortener

CMD ["urlshortener"]

