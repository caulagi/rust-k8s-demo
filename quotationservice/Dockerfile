FROM rust:1.48.0-slim-buster as build

COPY . /code
WORKDIR /code

# required for tonic-build
RUN rustup component add rustfmt
RUN cargo build

# RUN GRPC_HEALTH_PROBE_VERSION=v0.2.1 && \
#       wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
#       chmod +x /bin/grpc_health_probe
EXPOSE 50051

FROM debian:buster-slim
COPY --from=build /code/target/debug/quotation-server /usr/local/bin/quotation-server
ENTRYPOINT ["/usr/local/bin/quotation-server"]
