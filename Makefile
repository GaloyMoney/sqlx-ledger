build:
	cargo build

watch:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

next-watch:
	cargo watch -s 'cargo nextest run'

check-code:
	SQLX_OFFLINE=true cargo fmt --check --all
	SQLX_OFFLINE=true cargo clippy --all-features
	SQLX_OFFLINE=true cargo audit

test-in-ci:
	DATABASE_URL=postgres://user:password@postgres:5432/pg cargo sqlx migrate run
	SQLX_OFFLINE=true cargo nextest run --verbose

clean-deps:
	docker compose down

start-deps:
	docker compose up -d integration-deps

reset-deps: clean-deps start-deps setup-db

setup-db:
	cargo sqlx migrate run
