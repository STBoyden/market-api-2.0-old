FROM rust:alpine3.12
MAINTAINER sam@stboyden.com

WORKDIR /usr/src/market-api
COPY . .
RUN apk add --no-cache musl-dev
RUN apk add openssl-dev
RUN rustup toolchain install nightly
RUN cargo +nightly install --path .
CMD ["echo", "Running market-api..."]
CMD ["market-api"]
