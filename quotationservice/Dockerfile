FROM rust:1.81.0-slim-bookworm as build

COPY . /code
WORKDIR /code

# https://github.com/hyperium/tonic/issues/965
RUN apt update && apt install -y --no-install-recommends protobuf-compiler

# required for tonic-build
RUN rustup component add rustfmt
RUN cargo build

# RUN GRPC_HEALTH_PROBE_VERSION=v0.2.1 && \
#       wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
#       chmod +x /bin/grpc_health_probe
EXPOSE 9001

FROM debian:buster-slim
COPY --from=build /code/target/debug/quotation-server /usr/local/bin/quotation-server
ENTRYPOINT ["/usr/local/bin/quotation-server"]
