FROM rust:slim-buster AS builder
LABEL author="sam@stboyden.com" 

WORKDIR /usr/src/market-api
COPY . .
RUN apt update -y
RUN apt install libssl-dev libmariadbclient-dev-compat -y
RUN rustup toolchain install nightly --profile minimal
RUN cargo +nightly install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/market-api /usr/local/bin/market-api
RUN apt update -y
RUN apt upgrade -y
RUN apt install libmariadb3 -y
WORKDIR /usr/src/market-api
COPY ."env" .
WORKDIR /usr/src/market-api/migrations
COPY "migrations" .
WORKDIR /usr/src/market-api
CMD ["market-api"]
