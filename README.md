# rust_dynamodb_local
An example of how to test the integration of Rust with DynamoDb on your local development machine or in a Continuous Integration environment

## Requirements

* Docker Desktop up and running
* Rust toolchain

## macOS with Apple Silicon

### Additional requirements

* musl-cross with x86_64
```bash
brew install filosottile/musl-cross/musl-cross --with-x86_64
```

## Linux

### Additional requirements

* musl-gcc
```bash
sudo apt install musl-tools
```

## Run tests locally

```bash
./run.sh
```
