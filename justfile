build:
    cargo build

run:
    cargo run -p db-core

bench:
    cargo bench

check:
    cargo clippy
    cargo build

test:
    cargo nextest run

fmt:
    cargo fmt
