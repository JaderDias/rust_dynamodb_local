#!/bin/bash
docker-compose -f docker/test/docker-compose.yml kill && \
docker-compose -f docker/test/docker-compose.yml up \
  --build \
  --wait && \
CUSTOM_DOMAIN=example.com \
DYNAMODB_TABLE=table_name \
FIXED_UUID=abcdef1234567890abcdef1234567890 \
LOCAL_DYNAMODB_URL=http://localhost:8000 \
REGION=eu-west-1 \
cargo run