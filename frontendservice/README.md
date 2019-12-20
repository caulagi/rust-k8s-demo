# Frontend microservice

The client (browser) facing part of the application that provides
one http endpoint which returns the response from calling fortune microservice.
Run as -

```shell
$ FORTUNE_SERVICE_HOSTNAME=localhost RUST_LOG=frontend=debug cargo run
```
