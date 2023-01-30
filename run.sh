#!/bin/bash
mkdir -p dist/amazonlinux2
if uname -a | grep x86_64; then
  docker-compose -f docker/build/docker-compose.yml up \
    --exit-code-from amazonlinux2 || \
  exit
  cp target/release/rust_lambda dist/amazonlinux2/bootstrap
else
  rustup target install x86_64-unknown-linux-musl && \
  TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl || \
  exit
  cp target/x86_64-unknown-linux-musl/release/rust_lambda dist/amazonlinux2/bootstrap
fi
docker-compose -f docker/test/docker-compose.yml kill && \
docker-compose -f docker/test/docker-compose.yml up \
  --build \
  --wait && \
cargo run --example test
