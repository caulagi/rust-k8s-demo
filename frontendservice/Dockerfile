FROM rust:1.42.0-slim-buster as build

ENV RUST_LOG frontend=info
ENV FORTUNE_SERVICE_HOSTNAME fortuneservice

COPY . /code
WORKDIR /code

# required for tonic-build
RUN rustup component add rustfmt
RUN cargo build

EXPOSE 8080

FROM debian:buster-slim
COPY --from=build /code/target/debug/frontend-server /usr/local/bin/frontend-server
ENTRYPOINT ["/usr/local/bin/frontend-server"]
