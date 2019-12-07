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