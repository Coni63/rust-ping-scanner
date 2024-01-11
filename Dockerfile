FROM rust:1.75.0-slim-buster as builder

WORKDIR /app
COPY src/ /app/src
COPY Cargo.toml /app/Cargo.toml

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /app
COPY --from=builder /app/target/release/ /app

ENTRYPOINT [ "/app/ping-scanner" ]