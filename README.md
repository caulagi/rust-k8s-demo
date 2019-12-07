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

Setup a local kubernetes cluster. For example, using [kind][kind]

```shell
$ GO111MODULE="on" go get sigs.k8s.io/kind@v0.6.1
$ kind create cluster
```

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
[getent-osx]: https://github.com/petere/getent-osx/blob/master/getent
[fortune]: https://en.wikipedia.org/wiki/Fortune_%28Unix%29
[kind]: https://github.com/kubernetes-sigs/kind
