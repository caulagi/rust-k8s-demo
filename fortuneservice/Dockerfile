FROM rust:1.33-slim as build

ENV RUST_LOG fortune=debug
ENV FORTUNE_PATH /usr/games/fortune

COPY . /code
WORKDIR /code
RUN cargo build

RUN apt update && apt install -y --no-install-recommends wget fortunes fortune-mod
RUN GRPC_HEALTH_PROBE_VERSION=v0.2.1 && \
      wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
      chmod +x /bin/grpc_health_probe

EXPOSE 50051
ENTRYPOINT ["/code/target/debug/fortune-server"]
