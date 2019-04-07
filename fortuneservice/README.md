# Fortune service

A grpc server that calls the local 'fortune' binary and returns
the response. Run as -

```shell
$ FORTUNE_PATH=/usr/local/bin/fortune RUST_LOG=fortune=info cargo run
```
