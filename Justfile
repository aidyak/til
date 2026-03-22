default: check

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test

check: fmt-check lint test

build:
    cargo build

release:
    cargo build --release

run *args:
    cargo run -- {{args}}

clean:
    cargo clean

fix:
    cargo fmt
    cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged -- -D warnings

ci: check

help:
    @just --list
