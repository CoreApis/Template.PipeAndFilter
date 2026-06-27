FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.20 AS runtime
WORKDIR /app
COPY --from=build /app/target/release/pipe-and-filter /usr/local/bin/pipe-and-filter
COPY sample-orders.ndjson ./sample-orders.ndjson
# Reads NDJSON from stdin or a file argument; emits enriched NDJSON to stdout.
ENTRYPOINT ["pipe-and-filter"]
