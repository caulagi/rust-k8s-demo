FROM rust:1.33-slim as build

ENV FORTUNE_PATH=/usr/games/fortune

RUN mkdir /code
WORKDIR /code
COPY Cargo.toml Cargo.lock ./

COPY frontendservice/Cargo.toml frontendservice/Cargo.toml
COPY frontendservice/src/main.rs frontendservice/src/main.rs

COPY fortuneservice fortuneservice
WORKDIR fortuneservice
RUN cargo build

EXPOSE 50051
ENTRYPOINT ["/code/fortuneservice/target/server"]
