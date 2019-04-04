FROM rust:1.33-slim as build

WORKDIR /code
COPY Cargo.toml Cargo.lock ./

COPY fortuneservice/Cargo.toml fortuneservice/Cargo.toml
COPY fortuneservice/src/main.rs fortuneservice/src/main.rs

COPY frontendservice frontendservice
WORKDIR /code/frontendservice
RUN cargo build

EXPOSE 8080
ENTRYPOINT ["/code/frontendservice/target/server"]
