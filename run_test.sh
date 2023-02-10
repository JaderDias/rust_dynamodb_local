FUNCTION_NAME=$(aws --endpoint-url=http://localhost:4566  lambda list-functions | grep FunctionName | cut -d'"' -f4)
URL="http://localhost:4566/2015-03-31/functions/${FUNCTION_NAME}/invocations"

cargo run --example test "$URL" \
&& cargo fmt --all -- --check \
&& cargo clippy --all -- \
  -D clippy::all \
  -D clippy::pedantic \
  -D clippy::cargo \
  -D clippy::nursery \
  -W clippy::no_effect_underscore_binding \
  -W clippy::multiple_crate_versions \
  -W clippy::future_not_send