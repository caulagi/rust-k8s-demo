# rust-k8s-demo

[![Build & test project](https://github.com/caulagi/rust-k8s-demo/actions/workflows/ci.yml/badge.svg)](https://github.com/caulagi/rust-k8s-demo/actions/workflows/ci.yml)

![architecture][architecture]

This project is an experiment with how modern web applications would look like
when using **Rust** and **Kubernetes**. It is a simple web application that
returns a new quotation for each request.

There are two isolated microservices. The frontendservice provides one endpoint
that clients (browsers) can connect to. The quotationservice is a [grpc](https://grpc.io/) server,
that answers with a quotation for each request. Both the microservices
use fully asynchronous Rust libraries and are based on [tokio](https://tokio.rs/).


## Features

- [x] Microservices talking to each other using grpc
- [x] Local dev setup using skaffold
- [x] CI: Build code and run e2e tests for each commit in a k8s cluster

## Getting started

* The repo requires [git-lfs][git-lfs].

    ```shell
    $ git clone https://github.com/caulagi/rust-k8s-demo

    # make sure git lfs files have been cloned; otherwise git lfs pull will get the file
    $ ls -lh databaseservice/data
    total 15424
    -rw-r--r--  1 pradip.caulagi  staff   7.5M Feb 27 15:55 data.sql
    ```

* Setup a local kubernetes cluster using [kind](https://kind.sigs.k8s.io/).

    ```shell
    $ go install sigs.k8s.io/kind@v0.20.0
    $ kind create cluster --config kind-config.yaml
    ```

* Install [skaffold](https://skaffold.dev/) and run the application

    ```shell
    $ make bootstrap
    $ skaffold run --tail
    ```

**QED** - Go to [http://localhost](http://localhost). See [setup.md](setup.md) for more options for development setup.

## Todo

- [ ] Service mesh
- [ ] CD
- [ ] Distributed (open) tracing - partially done
- [ ] Prometheus, grafana, alert-manager
- [ ] Cert Manager
- [ ] External DNS(?)


## LICENSE

This project is licensed under [MIT](LICENSE).

[architecture]: https://user-images.githubusercontent.com/222507/96347681-a510fe00-10a3-11eb-8ed7-183c460b5def.png
[git-lfs]: https://git-lfs.github.com
