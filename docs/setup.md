# Local development setup

#### Setup local postgres database

```
# for fish shell, use (pwd) instead of $(pwd)
$ podman machine init rust-k8s-demo --volume $(pwd):/app

$ podman machine start rust-k8s-demo
$ podman system connection default rust-k8s-demo
$ podman run \
    -e POSTGRES_PASSWORD=1234 \
    -p 5432:5432 \
    -v /app/databaseservice/data/data.sql:/docker-entrypoint-initdb.d/01-data.sql \
    --name postgres \
    postgres:15
```

#### Using docker

```shell
$ podman build -t frontend frontendservice
$ podman build -t quotation quotationservice

# for linux: use localhost instead of docker.for.mac.localhost
$ podman run -it -p 8080:8080 \
    -e QUOTATION_SERVICE_HOSTNAME=host.containers.internal \
    -e RUST_LOG=frontend_server=debug,tower_http=trace \
    --name frontend \
    frontend
$ podman run -it -p 9001:9001 \
    -e POSTGRES_SERVICE=host.containers.internal \
    -e POSTGRES_PASSWORD=1234 \
    -e RUST_LOG=quotation_server=debug,tower_http=trace \
    --name quotation \
    quotation

# and goto http://localhost:8080
```

#### Local (no docker)

If you would like to run everything locally, you need the
[rust toolchain](https://rustup.rs/), of course.

```shell
$ cargo build
$ RUST_LOG=frontend_server=debug,tower_http=trace QUOTATION_SERVICE_HOSTNAME=localhost cargo run --bin frontend-server
$ RUST_LOG=quotation_server=debug,tower_http=trace POSTGRES_SERVICE=localhost POSTGRES_PASSWORD=1234 cargo run --bin quotation-server

# and goto http://localhost:8080
```
