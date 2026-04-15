build:
	cargo build

run:
	cargo run -p lsdb-cli

bench:
	cargo bench --bench insert -p lsdb-catalog
	cargo bench --bench query -p lsdb-execution

bench-insert:
	cargo bench --bench insert -p lsdb-catalog

bench-query:
	cargo bench --bench query -p lsdb-execution

check:
	cargo clippy --workspace
	cargo build --workspace

test:
	cargo nextest run --workspace --no-tests=pass

fmt:
	cargo fmt
