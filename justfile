build:
	cargo build

run:
	cargo run -p db-cli

bench:
	cargo bench --bench insert -p db-catalog
	cargo bench --bench query -p db-execution

bench-insert:
	cargo bench --bench insert -p db-catalog

bench-query:
	cargo bench --bench query -p db-execution

check:
	cargo clippy --workspace
	cargo build --workspace

test:
	cargo nextest run --workspace --no-tests=pass

fmt:
	cargo fmt
