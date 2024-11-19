FROM rust:latest AS rust-build

RUN apt-get update

WORKDIR /usr/src/scripts
COPY ./ ./
RUN cargo build --release



FROM node:20.11.1 AS node-build

WORKDIR /usr/src/
RUN git clone https://github.com/NPCDW/scripts-web.git
WORKDIR /usr/src/scripts-web
RUN npm install -g pnpm
RUN pnpm install
RUN pnpm run build



FROM debian:bookworm-slim

WORKDIR /scripts

COPY --from=node-build /usr/src/scripts-web/dist /ui
COPY --from=rust-build /usr/src/scripts/target/release/scripts /usr/local/bin/scripts

RUN apt-get update
RUN apt-get install -y openssl ca-certificates

ENTRYPOINT ["/usr/local/bin/scripts"]