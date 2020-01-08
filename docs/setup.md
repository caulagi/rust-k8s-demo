# Setting up kubernetes and local development

**Preparation (required)**: You need to have a postgres database with some quotations loaded.
You can follow the instructions at https://github.com/caulagi/postgres-quotation.

You can also **choose one of the options below** (they all do the same thing)
for setting up the project locally.


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
$ GO111MODULE="on" go get sigs.k8s.io/kind@v0.6.1
$ kind create cluster --config kind-config.yaml

# Run services
$ skaffold run

# Due to https://mauilion.dev/posts/kind-metallb/, it is better
# to port-forward the frontendservice pod
#
# requires jq binary
$ kubectl get pods --selector app=frontendservice -o json | jq  ".items[0].metadata.name" | xargs -I % kubectl port-forward pod/% 8080
```

[kind]: https://github.com/kubernetes-sigs/kind
