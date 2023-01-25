#!/bin/zsh
brew install filosottile/musl-cross/musl-cross --with-x86_64 && \
rustup target install x86_64-unknown-linux-musl && \
TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl && \
mkdir -p dist/x86_64-unknown-linux-musl && \
cp target/x86_64-unknown-linux-musl/release/rust_lambda dist/x86_64-unknown-linux-musl/bootstrap && \
mkdir -p docker/dynamodb && \
chmod -R 775 docker && \
docker-compose kill && \
docker-compose up --build -d && \
cargo run --example test
