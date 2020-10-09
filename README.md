# rust-k8s-demo

[![Build Status][actions-badge]][actions-url]

![architecture][architecture]

This project is an experiment with how modern web applications would look like
when using **Rust** and **Kubernetes**. It is inspired by [microservices-demo][demo].

It is a simple web application that returns a new quotation for each request.

There are two isolated microservices. The frontendservice provides one endpoint
that clients (browsers) can connect to. The quotationservice is a [grpc](https://grpc.io/) server,
that answers with a quotation for each request. Both the microservices
use fully asynchronous Rust libraries and are based on [tokio](https://tokio.rs/).


## Features

- [x] Microservices talking to each other using grpc
- [x] Local dev setup using skaffold
- [x] CI
- [ ] Service mesh

## Getting started

* The repo requires [git-lfs][git-lfs].

    ```shell
    $ git clone https://github.com/caulagi/rust-k8s-demo

    # make sure git lfs files have been cloned
    # otherwise git lfs pull will get the file
    $ ls -lh databaseservice/data
    total 15424
    -rw-r--r--  1 pradip.caulagi  staff   7.5M Feb 27 15:55 data.sql
    ```

* Setup a local kubernetes cluster, using Kubernetes for docker-for-mac
(for Linux or other options in getting started, see [setup](./docs/setup.md)).

* Install [skaffold](https://skaffold.dev/) and run the application

    ```shell
    $ make bootstrap
    $ skaffold run --tail
    ```

* **QED** - Go to [http://localhost](http://localhost)


## Todo

- [ ] CD
- [ ] Distributed (open) tracing
- [ ] Prometheus, grafana, alert-manager
- [ ] Cert Manager
- [ ] External DNS(?)


## LICENSE

This project is licensed under [MIT](LICENSE).

[demo]: https://github.com/GoogleCloudPlatform/microservices-demo
[actions-badge]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fcaulagi%2Frust-k8s-demo%2Fbadge&style=flat&label=Build%20status
[actions-url]: https://actions-badge.atrox.dev/caulagi/rust-k8s-demo/goto
[architecture]: https://user-images.githubusercontent.com/222507/72002857-89411780-3248-11ea-9e16-9e6912f2a75b.png
[git-lfs]: https://git-lfs.github.com
