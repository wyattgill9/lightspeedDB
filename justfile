build:
    cargo build

run:
    cargo run -p db-core

check:
    cargo clippy
    cargo build

test:
    cargo nextest run

fmt:
    cargo fmt
