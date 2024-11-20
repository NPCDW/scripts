FROM rust:latest AS rust-build

RUN apt-get update

WORKDIR /usr/src/scripts
COPY ./ ./
RUN cargo build --release



FROM debian:bookworm-slim

WORKDIR /scripts

RUN apt-get update
RUN apt-get install -y openssl ca-certificates

ENTRYPOINT ["/usr/local/bin/scripts"]