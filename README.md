# fortune-k8s-demo

[![Build Status][azure-badge]][azure-url]


**TL;DR** `skaffold run --tail` and go to [localhost](http://localhost)

This project is an experiment with how modern web applications would look like
when using **Rust** and **Kubernetes**. It is based on [microservices-demo][demo].

## Features

- [x] Microservices talking to each other using grpc
- [x] Local dev setup using skaffold
- [x] CI (done)/CD (todo)
- [ ] Service mesh and open tracing (todo)

## Local development

**Use one of the options below**. They all do the same thing.

#### [Skaffold](https://skaffold.dev/)

This is the simplest option to try out the project. Follow the guide from [demo][demo]
project to setup kubernetes for docker and skaffold.

```shell
$ skaffold run --tail

# goto http://localhost
```

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
$ FORTUNE_SERVICE_HOSTNAME=localhost GETENT_PATH=~/bin/getent RUST_LOG=frontend=debug cargo run

$ cd fortuneservice
$ FORTUNE_PATH=/usr/local/bin/fortune cargo run

# and goto http://localhost:8080
```

## LICENSE

This project is licensed under [MIT](LICENSE).


[azure-badge]: https://dev.azure.com/caulagi/fortune-k8s-demo/_apis/build/status/caulagi.fortune-k8s-demo?branchName=master
[azure-url]: https://dev.azure.com/caulagi/fortune-k8s-demo/_build/latest?definitionId=1&branchName=master
[demo]: https://github.com/GoogleCloudPlatform/microservices-demo
[getent-osx]: https://github.com/petere/getent-osx/blob/master/getent
