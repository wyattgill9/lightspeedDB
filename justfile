build:
	cargo build

run:
	cargo run -p db-cli

bench:
	cargo bench --bench insert -p db-catalog

check:
	cargo clippy --workspace
	cargo build --workspace

test:
	cargo nextest run --workspace --no-tests=pass

fmt:
	cargo fmt
