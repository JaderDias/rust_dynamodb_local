.PHONY: all clean run test check scan_table integration_test unit_test clippy watch
run:
	docker-compose -f docker/test/docker-compose.yml kill
	docker-compose -f docker/test/docker-compose.yml up --build --detach
	RUSTFLAGS="-C instrument-coverage" \
		cargo build
	CUSTOM_DOMAIN=example.com \
		DYNAMODB_TABLE=table_name \
		LOCAL_DYNAMODB_URL=http://localhost:8000 \
		REGION=eu-west-1 \
		./target/debug/rust_lambda

test: unit_test integration_test

check: integration_test clippy
	cargo fmt --all -- --check

scan_table:
	@if ! grep -F '[profile localhost]' <~/.aws/config; then \
		echo "[profile localhost]\nregion = us-east-1" >>~/.aws/config; \
	fi
	@if ! grep -F '[localhost]' <~/.aws/credentials; then \
		echo "[localhost]\naws_access_key_id = ANY_ACCESS_KEY_WILL_DO\naws_secret_access_key = ANY_SECRET_KEY_WILL_DO" >>~/.aws/credentials; \
	fi
	aws dynamodb scan --table-name table_name --endpoint-url http://localhost:8000 --profile localhost

integration_test:
	LOCAL_DYNAMODB_URL=http://localhost:8000 \
		cargo run --example test http://localhost:8080

unit_test:
	cargo test

clippy:
	cargo clippy --all -- \
		-D "clippy::all" \
		-D clippy::pedantic \
		-D clippy::cargo \
		-D clippy::nursery \
		-W clippy::no_effect_underscore_binding \
		-W clippy::multiple_crate_versions \
		-W clippy::future_not_send

watch:
	cargo watch --clear