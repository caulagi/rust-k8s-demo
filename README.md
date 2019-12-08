# fortune-k8s-demo

[![Build Status][azure-badge]][azure-url]


**TL;DR** `skaffold run --tail` and go to [localhost](http://localhost)

This project is an experiment with how modern web applications would look like
when using **Rust** and **Kubernetes**. It is based on [microservices-demo][demo].

It is a simple web application that returns a new [fortune cookie][fortune] for each request.

There are two isolated microservices. The frontendservice provides one endpoint
that clients (browsers) can connect to. The fortuneservice is a [grpc](https://grpc.io/) server,
that answers with 'fortune cookies' for each request. Both the microservices
use fully asynchronous Rust libraries and are based on [tokio](https://tokio.rs/).


## Features

- [x] Microservices talking to each other using grpc
- [x] Local dev setup using skaffold
- [x] CI (done)/CD (todo)
- [ ] Service mesh and open tracing (todo)

## Getting started

Setup a local kubernetes cluster, using Kubernetes for docker-for-mac
(for Linux, see [CONTRIBUTING][CONTRIBUTING]).

Install [skaffold](https://skaffold.dev/) and run the application

```shell
$ skaffold run --tail
```

**QED** - Go to [http://localhost](http://localhost)

## LICENSE

This project is licensed under [MIT](LICENSE).


[azure-badge]: https://dev.azure.com/caulagi/fortune-k8s-demo/_apis/build/status/caulagi.fortune-k8s-demo?branchName=master
[azure-url]: https://dev.azure.com/caulagi/fortune-k8s-demo/_build/latest?definitionId=1&branchName=master
[demo]: https://github.com/GoogleCloudPlatform/microservices-demo
[fortune]: https://en.wikipedia.org/wiki/Fortune_%28Unix%29
[CONTRIBUTING]: ./.github/CONTRIBUTING.md
