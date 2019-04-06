FROM rust:1.33-slim as build

ENV RUST_LOG frontend=debug
ENV GETENT_PATH /usr/bin/getent
ENV FORTUNE_SERVICE_HOSTNAME fortuneservice

COPY . /code
WORKDIR /code
RUN cargo build

EXPOSE 8080
ENTRYPOINT ["/code/target/debug/frontend-server"]
