FROM rust:1.39.0-slim-buster as build

ENV RUST_LOG frontend=info
ENV GETENT_PATH /usr/bin/getent
ENV FORTUNE_SERVICE_HOSTNAME fortuneservice

COPY . /code
WORKDIR /code
RUN cargo build

EXPOSE 8080
ENTRYPOINT ["/code/target/debug/frontend-server"]
