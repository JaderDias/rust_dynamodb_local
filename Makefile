.PHONY: all clean test check scan_table unit_test clippy watch refresh_database
test: refresh_database
	rustup update nightly
	rustup default nightly
	CARGO_INCREMENTAL=0 \
		RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
		RUSTDOCFLAGS="-Cpanic=abort" \
		cargo build --all-targets
	cargo test
	CUSTOM_DOMAIN=example.com \
		DYNAMODB_TABLE=table_name \
		LOCAL_DYNAMODB_URL=http://localhost:8000 \
		REGION=eu-west-1 \
		./target/debug/rust_lambda &
	LOCAL_DYNAMODB_URL=http://localhost:8000 \
		./target/debug/examples/test http://localhost:8080
	pkill rust_lambda

check: clippy
	cargo fmt --all -- --check

scan_table:
	@if ! grep -F '[profile localhost]' <~/.aws/config; then \
		echo "[profile localhost]\nregion = us-east-1" >>~/.aws/config; \
	fi
	@if ! grep -F '[localhost]' <~/.aws/credentials; then \
		echo "[localhost]\naws_access_key_id = ANY_ACCESS_KEY_WILL_DO\naws_secret_access_key = ANY_SECRET_KEY_WILL_DO" >>~/.aws/credentials; \
	fi
	aws dynamodb scan --table-name table_name --endpoint-url http://localhost:8000 --profile localhost

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

refresh_database:
	docker-compose -f docker/test/docker-compose.yml kill
	docker-compose -f docker/test/docker-compose.yml up --build --detach