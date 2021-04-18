FROM rust:alpine3.12 AS builder
MAINTAINER sam@stboyden.com

WORKDIR /usr/src/market-api
COPY . .
RUN apk add --no-cache musl-dev openssl-dev mariadb-dev
RUN rustup toolchain install nightly
RUN cargo +nightly install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/market-api /usr/local/bin/market-api
WORKDIR /usr/src/market-api
COPY .env .
RUN env
CMD ["market-api"]
