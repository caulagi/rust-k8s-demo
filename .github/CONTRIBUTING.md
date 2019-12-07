# Contributing to fortune-k8s-demo

Thank you for helping us improve the project.

## Getting started

The [README](../README.md) has instructions on how to run this project.
You can also **choose one of the options below** (they all do the same thing).

#### Using docker

```shell
$ docker build -t frontend frontendservice
$ docker build -t fortune fortuneservice

# for linux: FORTUNE_SERVICE_HOSTNAME=localhost
$ docker run -it -p 8080:8080 -e FORTUNE_SERVICE_HOSTNAME=docker.for.mac.localhost frontend
$ docker run -it -p 50051:50051 fortune

# and goto http://localhost:8080
```

#### Really local

You need the [rust toolchain](https://rustup.rs/) and [fortune](https://en.wikipedia.org/wiki/Fortune_%28Unix%29), of course.
You also need [getent](https://en.wikipedia.org/wiki/Getent) (for now). On OSX,
you can put [this file][getent-osx] in your path as the binary.

```shell
$ cd frontendservice
$ FORTUNE_SERVICE_HOSTNAME=localhost GETENT_PATH=~/bin/getent RUST_LOG=frontend=info cargo run

$ cd fortuneservice
$ FORTUNE_PATH=/usr/local/bin/fortune RUST_LOG=fortune=info cargo run

# and goto http://localhost:8080
```

## Local Kubernetes

You can setup a local kubernetes on Linux and Mac using [kind][kind]

```shell
$ GO111MODULE="on" go get sigs.k8s.io/kind@v0.6.1
$ kind create cluster

# Required due to https://github.com/GoogleContainerTools/skaffold/issues/3296
$ kubectl config rename-context "kind-kind" "kind-kind@kind"

# Due to https://mauilion.dev/posts/kind-metallb/, it is better
# to port-forward the frontendservice pod
#
# requires jq binary
$ kubectl get pods --selector app=frontendservice -o json | jq  ".items[0].metadata.name" | xargs -I % kubectl port-forward pod/% 8080
```

[kind]: https://github.com/kubernetes-sigs/kind
