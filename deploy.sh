#!/bin/bash
S3_BUCKET=${1}
if [ -z "$S3_BUCKET" ]
then
    echo "Usage: deploy.sh <s3_bucket>"
    echo "   or: deploy.sh <s3_bucket> --endpoint-url=http://localhost:4566"
    exit 1
fi

ENDPOINT=${2}
if [ -n "$ENDPOINT" ]
then
    localstack stop
    DYNAMODB_IN_MEMORY=1 localstack start &
    if ! cat ~/.aws/config | grep -F '[profile localhost]'; then
      echo -e "[profile localhost]\nregion = us-east-1" >> ~/.aws/config
    fi
    if ! cat ~/.aws/credentials | grep -F '[localhost]'; then
      echo -e "[localhost]\naws_access_key_id = NOT_NEEDED\naws_secret_access_key = NOT_NEEDED" >> ~/.aws/credentials
    fi
    ENDPOINT="$ENDPOINT --profile localhost"
    aws $ENDPOINT s3api create-bucket --bucket $S3_BUCKET
fi

mkdir -p dist/amazonlinux2 \
|| exit 1
if uname -a | grep x86_64; then
  docker-compose -f docker/build/docker-compose.yml up \
    --exit-code-from amazonlinux2 \
  || exit 1
  cp target/release/rust_lambda dist/amazonlinux2/bootstrap
else
  # Faster compilation on Mac Apple Silicon
  rustup target install x86_64-unknown-linux-musl \
  && TARGET_CC=x86_64-linux-musl-gcc \
  RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
  cargo build --release --target x86_64-unknown-linux-musl \
  || exit 1
  cp target/x86_64-unknown-linux-musl/release/rust_lambda dist/amazonlinux2/bootstrap
fi
rm dist/amazonlinux2.zip
zip -jr dist/amazonlinux2.zip dist/amazonlinux2 \
|| exit 1
if uname -a | grep Darwin; then
  LAMBDA_CODE_OBJECT_KEY=`md5 dist/amazonlinux2.zip | cut -d' ' -f4`
else
  LAMBDA_CODE_OBJECT_KEY=`md5sum dist/amazonlinux2.zip | cut -d' ' -f1`
fi
aws $ENDPOINT s3 cp dist/amazonlinux2.zip "s3://$S3_BUCKET/$LAMBDA_CODE_OBJECT_KEY" \
&& aws $ENDPOINT localhost cloudformation deploy \
  --template-file cloudformation.yml \
  --stack-name "rust-lambda" \
  --parameter-overrides "LambdaCodeBucket=$S3_BUCKET" "LambdaCodeObjectKey=$LAMBDA_CODE_OBJECT_KEY" \
  --capabilities CAPABILITY_IAM
