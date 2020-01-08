# Fortune service

A grpc server that gets a random quotation from database and returns
the response. Run as -

```shell
$ RUST_LOG=fortune=debug POSTGRES_SERVICE=localhost POSTGRES_PASSWORD=1234 cargo run
```
