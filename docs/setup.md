# Setting up kubernetes and local development

#### Setup local postgres database

```
$ docker run -d -e POSTGRES_PASSWORD=1234 -p 5432:5432 --mount type=bind,source=$(pwd)/databaseservice/data/data.sql,target=/docker-entrypoint-initdb.d/01-data.sql postgres:12
```

#### Using docker

```shell
$ docker build -t frontend frontendservice
$ docker build -t quotation quotationservice

# for linux: use localhost instead of docker.for.mac.localhost
$ docker run -it -p 8080:8080 -e QUOTATION_SERVICE_HOSTNAME=docker.for.mac.localhost frontend
$ docker run -it -p 50051:50051 -e POSTGRES_SERVICE=docker.for.mac.localhost -e POSTGRES_PASSWORD=1234 quotation

# and goto http://localhost:8080
```

#### Local (no docker)

If you would like to run everything locally, you need the
[rust toolchain](https://rustup.rs/), of course.

```shell
$ cd frontendservice
$ QUOTATION_SERVICE_HOSTNAME=localhost RUST_LOG=frontend=info cargo run

$ cd quotationservice
$ RUST_LOG=quotation=info POSTGRES_SERVICE=localhost POSTGRES_PASSWORD=1234 cargo run

# and goto http://localhost:8080
```

#### Kind kubernetes cluster

You can setup a local kubernetes on Linux and Mac using [kind][kind]

```shell
$ GO111MODULE="on" go get sigs.k8s.io/kind@v0.7.0
$ kind create cluster --config kind-config.yaml

$ make bootstrap
$ skaffold run

**QED** goto http://localhost
```

[kind]: https://github.com/kubernetes-sigs/kind
