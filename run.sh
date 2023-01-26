#!/bin/bash
mkdir -p dist/x86_64-unknown-linux-musl
if uname -a | grep x86_64; then
    cargo build --release && \
    cp target/release/rust_lambda dist/x86_64/bootstrap
else
    rustup target install x86_64-unknown-linux-musl && \
    TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/release/rust_lambda dist/x86_64/bootstrap
fi
docker-compose kill && \
docker-compose up --build -d && \
cargo run --example test
