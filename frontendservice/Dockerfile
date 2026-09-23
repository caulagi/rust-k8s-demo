FROM rust:1.98.1-trixie AS build

ENV RUST_LOG frontend=info
ENV FORTUNE_SERVICE_HOSTNAME fortuneservice

COPY . /code
WORKDIR /code

# https://github.com/hyperium/tonic/issues/965
RUN apt update && apt install -y --no-install-recommends protobuf-compiler

# required for tonic-build
RUN rustup component add rustfmt
RUN cargo build

EXPOSE 8080

FROM debian:trixie
COPY --from=build /code/target/debug/frontend-server /usr/local/bin/frontend-server
ENTRYPOINT ["/usr/local/bin/frontend-server"]
