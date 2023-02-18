# rust_dynamodb_local
[![build status](https://github.com/JaderDias/rust_dynamodb_local/workflows/Rust/badge.svg)](https://github.com/JaderDias/rust_dynamodb_local/actions?query=workflow%3ARust)
[![dependency status](https://deps.rs/repo/github/JaderDias/rust_dynamodb_local/status.svg)](https://deps.rs/repo/github/JaderDias/rust_dynamodb_local)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/JaderDias/rust_dynamodb_local.svg)](http://isitmaintained.com/project/JaderDias/rust_dynamodb_local "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/JaderDias/rust_dynamodb_local.svg)](http://isitmaintained.com/project/JaderDias/rust_dynamodb_local "Percentage of issues still open")
[![Code coverage](https://codecov.io/gh/JaderDias/rust_dynamodb_local/branch/main/graph/badge.svg)](https://codecov.io/gh/JaderDias/rust_dynamodb_local)

An example of how to test the integration of Rust with DynamoDb on your local development machine or in a Continuous Integration environment

## Supported hosts

* Linux
* MacOS

## Requirements

### Development & testing

* Docker Desktop up and running
* docker-compose
* gcc
* Rust toolchain

### additional deployment requirements

* AWS Command Line Interface

### additional macOS with Apple Silicon requirements

* musl-cross with x86_64
```bash
brew install filosottile/musl-cross/musl-cross --with-x86_64
```

## Run tests locally

```bash
make test
```

## Run tests as GitHub actions

.github/workflows/rust.yml
