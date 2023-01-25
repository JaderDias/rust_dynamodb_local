# rust_local_dynamodb
An example of how to test the integration of Rust with DynamoDb on your local development machine or in a Continuous Integration environment

## macOS

### Requirements

* Docker Desktop up and running
* Rust toolchain
* musl-cross with x86_64
```
brew install filosottile/musl-cross/musl-cross --with-x86_64
```

### Run tests locally

```zsh
./run.sh
```