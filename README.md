# rust_dynamodb_local
An example of how to test the integration of Rust with DynamoDb on your local development machine or in a Continuous Integration environment

## Supported hosts

* Linux
* MacOS

## Requirements

* Docker Desktop up and running
* docker-compose
* gcc
* Rust toolchain 

### macOS with Apple Silicon additional requirements

* musl-cross with x86_64
```bash
brew install filosottile/musl-cross/musl-cross --with-x86_64
```

## Run tests locally

```bash
./run.sh
```

on another terminal

```bash
./run_test.sh
```

## Run tests as GitHub actions

.github/workflows/rust.yml
