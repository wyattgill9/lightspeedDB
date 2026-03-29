build:
    cargo build

run:
    cargo run

check:
    cargo clippy
    cargo build

test:
    cargo nextest run

fmt:
    cargo fmt
