#!/bin/bash
mkdir -p dist/amazonlinux2 && \
docker-compose -f docker/test-docker-compose.yml kill && \
docker-compose -f docker/build-docker-compose.yml up --build && \
cp target/release/rust_lambda dist/amazonlinux2/bootstrap && \
docker-compose -f docker/test-docker-compose.yml up --build -d && \
cargo run --example test
